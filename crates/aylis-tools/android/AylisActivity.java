package dev.aylis;

import android.app.NativeActivity;
import android.content.Context;
import android.os.Bundle;
import android.text.Editable;
import android.text.InputType;
import android.text.Selection;
import android.view.KeyEvent;
import android.view.View;
import android.view.ViewGroup;
import android.view.Window;
import android.view.WindowInsets;
import android.view.WindowInsetsController;
import android.view.WindowManager;
import android.os.Build;
import android.graphics.PixelFormat;
import android.view.Gravity;
import android.webkit.JavascriptInterface;
import android.webkit.WebResourceRequest;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import java.util.HashMap;
import android.view.inputmethod.BaseInputConnection;
import android.view.inputmethod.EditorInfo;
import android.view.inputmethod.InputConnection;
import android.view.inputmethod.InputMethodManager;

/**
 * Gerado pelo aylis. A NativeActivity com o teclado virtual completo: sugestões, correção
 * e emoji chegam como edições de texto (apagar N antes do cursor, M depois, inserir) e o
 * aylis aplica no controle com foco. O resto do app continua em Rust (android_main).
 */
public class AylisActivity extends NativeActivity {
    // Registrados pelo aylis (RegisterNatives) quando o android_main começa.
    static native void nativeEdit(int before, int after, String text);
    static native void nativeKey(int keyCode);
    /** Eventos do WebView: kind 0 = carregou, 1 = vai navegar, 2 = mensagem da página. */
    static native void nativeWebEvent(long id, int kind, String text);
    static native void nativeNotification(long component, String action);
    /** Resposta do seletor de documentos: os caminhos (ou URIs, ao salvar) separados por \n; vazio = cancelou. */
    static native void nativeFilesPicked(long request, String paths);
    /** Resposta da biometria: 0 confirmou, 1 cancelou, 2 falhou (com o motivo). */
    static native void nativeBiometricResult(long request, int code, String message);
    /** O que chegou de fora (o AppLink): kind 0 = link, 1 = compartilhado (texto ou caminhos separados por ;). */
    static native void nativeIncoming(int kind, String text);
    /** Resposta de um pedido a outro app: ok 1 = RESULT_OK, com o que ele devolveu. */
    static native void nativeLinkResult(long request, int ok, String text);

    private InputView input;
    private final HashMap<Long, WebView> webviews = new HashMap<>();

    @Override
    protected void onCreate(Bundle state) {
        super.onCreate(state);
        current = this;
        input = new InputView(this);
        addContentView(input, new ViewGroup.LayoutParams(1, 1));
        // O link ou o compartilhar que abriu o app: entregue quando o aylis registrar os
        // nativos (deliverPending), que ainda não existem aqui.
        pendingIntent = getIntent();
        // O toque numa notificação push que abriu o app.
        pushOpened(pendingIntent);
        // Aberto por uma ação (WakeAppAction, NotifyAction): por cima do bloqueio e o evento.
        actionOpened(pendingIntent);
    }

    // ----- AppLink: abrir endereços e apps, compartilhar, pedir, receber -----
    private android.content.Intent pendingIntent;
    private static final int LINK_REQUEST = 7103;
    private long linkRequest;

    /** O aylis chama depois de registrar os nativos: entrega o que abriu o app. */
    public void deliverPending() {
        android.content.Intent intent = pendingIntent;
        pendingIntent = null;
        if (intent != null) {
            deliver(intent);
        }
    }

    /** Um link (ACTION_VIEW) ou o que outro app compartilhou (ACTION_SEND...) vai para o aylis. */
    private void deliver(final android.content.Intent intent) {
        String action = intent.getAction();
        if (android.content.Intent.ACTION_VIEW.equals(action) && intent.getDataString() != null) {
            nativeIncoming(0, intent.getDataString());
            return;
        }
        if (!android.content.Intent.ACTION_SEND.equals(action) && !android.content.Intent.ACTION_SEND_MULTIPLE.equals(action)) {
            return;
        }
        final java.util.ArrayList<android.net.Uri> uris = new java.util.ArrayList<android.net.Uri>();
        if (android.content.Intent.ACTION_SEND_MULTIPLE.equals(action)) {
            java.util.ArrayList<android.os.Parcelable> list = intent.getParcelableArrayListExtra(android.content.Intent.EXTRA_STREAM);
            if (list != null) {
                for (android.os.Parcelable p : list) {
                    uris.add((android.net.Uri) p);
                }
            }
        } else {
            android.os.Parcelable p = intent.getParcelableExtra(android.content.Intent.EXTRA_STREAM);
            if (p != null) {
                uris.add((android.net.Uri) p);
            }
        }
        if (uris.isEmpty()) {
            CharSequence text = intent.getCharSequenceExtra(android.content.Intent.EXTRA_TEXT);
            nativeIncoming(1, text == null ? "" : text.toString());
            return;
        }
        // Os arquivos viram cópias no cache (o URI de outro app não dura): fora da tela.
        new Thread(new Runnable() {
            @Override
            public void run() {
                StringBuilder paths = new StringBuilder();
                java.io.File dir = new java.io.File(getCacheDir(), "received/" + System.currentTimeMillis());
                dir.mkdirs();
                for (android.net.Uri uri : uris) {
                    java.io.File out = new java.io.File(dir, displayName(uri));
                    try (java.io.InputStream in = getContentResolver().openInputStream(uri);
                         java.io.OutputStream os = new java.io.FileOutputStream(out)) {
                        byte[] buf = new byte[1 << 16];
                        int n;
                        while ((n = in.read(buf)) > 0) {
                            os.write(buf, 0, n);
                        }
                        if (paths.length() > 0) {
                            paths.append(';');
                        }
                        paths.append(out.getAbsolutePath());
                    } catch (Exception e) {
                        // O que não deu para copiar fica de fora.
                    }
                }
                nativeIncoming(1, paths.toString());
            }
        }).start();
    }

    public boolean openUrl(String url) {
        try {
            android.content.Intent intent = new android.content.Intent(android.content.Intent.ACTION_VIEW, android.net.Uri.parse(url));
            startActivity(intent);
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    /** Abre o app do pacote (no Android 11+, o manifesto precisa listá-lo em <queries>). */
    public boolean openApp(String pkg) {
        android.content.Intent intent = getPackageManager().getLaunchIntentForPackage(pkg);
        if (intent == null) {
            return false;
        }
        try {
            startActivity(intent);
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    /** A folha de compartilhar do sistema: o texto e cópias dos arquivos, lidas pelo Files. */
    public boolean share(String text, String[] paths) {
        try {
            java.util.ArrayList<android.net.Uri> uris = new java.util.ArrayList<android.net.Uri>();
            java.io.File dir = new java.io.File(getCacheDir(), "shared");
            dir.mkdirs();
            for (String path : paths) {
                java.io.File from = new java.io.File(path);
                java.io.File to = new java.io.File(dir, from.getName());
                try (java.io.InputStream in = new java.io.FileInputStream(from);
                     java.io.OutputStream os = new java.io.FileOutputStream(to)) {
                    byte[] buf = new byte[1 << 16];
                    int n;
                    while ((n = in.read(buf)) > 0) {
                        os.write(buf, 0, n);
                    }
                }
                uris.add(android.net.Uri.parse("content://" + getPackageName() + ".aylis.files/" + android.net.Uri.encode(to.getName())));
            }
            android.content.Intent intent;
            if (uris.size() > 1) {
                intent = new android.content.Intent(android.content.Intent.ACTION_SEND_MULTIPLE);
                intent.putParcelableArrayListExtra(android.content.Intent.EXTRA_STREAM, uris);
                intent.setType("*/*");
            } else if (uris.size() == 1) {
                intent = new android.content.Intent(android.content.Intent.ACTION_SEND);
                intent.putExtra(android.content.Intent.EXTRA_STREAM, uris.get(0));
                intent.setType(Files.mime(uris.get(0).getLastPathSegment()));
            } else {
                intent = new android.content.Intent(android.content.Intent.ACTION_SEND);
                intent.setType("text/plain");
            }
            if (text != null && !text.isEmpty()) {
                intent.putExtra(android.content.Intent.EXTRA_TEXT, text);
            }
            if (!uris.isEmpty()) {
                android.content.ClipData clip = android.content.ClipData.newRawUri("", uris.get(0));
                for (int i = 1; i < uris.size(); i++) {
                    clip.addItem(new android.content.ClipData.Item(uris.get(i)));
                }
                intent.setClipData(clip);
                intent.addFlags(android.content.Intent.FLAG_GRANT_READ_URI_PERMISSION);
            }
            startActivity(android.content.Intent.createChooser(intent, null));
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    // ----- Loopback: o som que os outros apps tocam (AudioPlaybackCapture) -----
    private static final int LOOPBACK_REQUEST = 7401;
    /** 0 parado, 1 pedindo, 2 gravando, 3 negado. */
    private volatile int loopbackState;
    static volatile android.media.AudioRecord loopback;
    static volatile android.content.Intent projectionData;
    static volatile int projectionResult;

    /** Começa (pede a autorização na primeira vez); devolve o estado. */
    public int loopbackStart() {
        if (loopback != null) {
            return 2;
        }
        if (loopbackState == 1 || loopbackState == 3) {
            return loopbackState;
        }
        if (!hasPermission("android.permission.RECORD_AUDIO")) {
            ensurePermission("android.permission.RECORD_AUDIO");
            return 1;
        }
        loopbackState = 1;
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.media.projection.MediaProjectionManager m = (android.media.projection.MediaProjectionManager) getSystemService(Context.MEDIA_PROJECTION_SERVICE);
                startActivityForResult(m.createScreenCaptureIntent(), LOOPBACK_REQUEST);
            }
        });
        return 1;
    }

    /** As amostras que chegaram (estéreo intercalado); 0 nenhuma. */
    public int loopbackRead(short[] buffer) {
        android.media.AudioRecord r = loopback;
        if (r == null) {
            return 0;
        }
        int n = r.read(buffer, 0, buffer.length, android.media.AudioRecord.READ_NON_BLOCKING);
        return Math.max(n, 0);
    }

    public void loopbackStop() {
        android.media.AudioRecord r = loopback;
        loopback = null;
        if (r != null) {
            r.stop();
            r.release();
        }
        loopbackState = 0;
        stopService(new android.content.Intent(this, Projection.class));
    }

    /**
     * O serviço da captura: desde o Android 10 a gravação do som dos outros apps precisa de um
     * serviço em primeiro plano do tipo mediaProjection rodando enquanto dura.
     */
    public static class Projection extends android.app.Service {
        @Override
        public android.os.IBinder onBind(android.content.Intent intent) {
            return null;
        }

        @Override
        public int onStartCommand(android.content.Intent intent, int flags, int startId) {
            android.app.NotificationManager nm = (android.app.NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
            nm.createNotificationChannel(new android.app.NotificationChannel("aylis.loopback", "Captura do som", android.app.NotificationManager.IMPORTANCE_LOW));
            int icon = getApplicationInfo().icon != 0 ? getApplicationInfo().icon : android.R.drawable.ic_btn_speak_now;
            android.app.Notification n = new android.app.Notification.Builder(this, "aylis.loopback")
                .setContentTitle("Gravando o som dos apps").setSmallIcon(icon).setOngoing(true).build();
            AylisActivity a = current;
            try {
                startForeground(7402, n, android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PROJECTION);
                android.media.projection.MediaProjectionManager m = (android.media.projection.MediaProjectionManager) getSystemService(Context.MEDIA_PROJECTION_SERVICE);
                android.media.projection.MediaProjection p = m.getMediaProjection(projectionResult, projectionData);
                android.media.AudioPlaybackCaptureConfiguration config = new android.media.AudioPlaybackCaptureConfiguration.Builder(p)
                    .addMatchingUsage(android.media.AudioAttributes.USAGE_MEDIA)
                    .addMatchingUsage(android.media.AudioAttributes.USAGE_GAME)
                    .addMatchingUsage(android.media.AudioAttributes.USAGE_UNKNOWN)
                    .build();
                android.media.AudioFormat format = new android.media.AudioFormat.Builder()
                    .setEncoding(android.media.AudioFormat.ENCODING_PCM_16BIT)
                    .setSampleRate(48000)
                    .setChannelMask(android.media.AudioFormat.CHANNEL_IN_STEREO)
                    .build();
                android.media.AudioRecord r = new android.media.AudioRecord.Builder()
                    .setAudioFormat(format)
                    .setBufferSizeInBytes(48000 * 4)
                    .setAudioPlaybackCaptureConfig(config)
                    .build();
                r.startRecording();
                loopback = r;
                if (a != null) {
                    a.loopbackState = 2;
                }
            } catch (Exception e) {
                if (a != null) {
                    a.loopbackState = 3;
                }
                stopSelf();
            }
            return START_NOT_STICKY;
        }
    }

    // ----- Nfc: ler e gravar tags NDEF (o modo leitor, só com o app na frente) -----
    // Os eventos vão para uma fila que o aylis lê (nfcPoll), um por linha, campos com tab:
    // "tag id tecnologia gravável ndef-hex", "written" e "error mensagem".
    private final java.util.concurrent.ConcurrentLinkedQueue<String> nfcEvents = new java.util.concurrent.ConcurrentLinkedQueue<String>();
    private volatile boolean nfcReading;
    private volatile byte[] nfcPendingWrite;
    private boolean resumed;

    private static String hex(byte[] bytes) {
        if (bytes == null) {
            return "";
        }
        StringBuilder sb = new StringBuilder();
        for (byte b : bytes) {
            sb.append(String.format("%02X", b & 0xFF));
        }
        return sb.toString();
    }

    private static byte[] unhex(String text) {
        int n = text.length() / 2;
        byte[] out = new byte[n];
        for (int i = 0; i < n; i++) {
            out[i] = (byte) Integer.parseInt(text.substring(i * 2, i * 2 + 2), 16);
        }
        return out;
    }

    /** Uma linha só, sem tab (os campos dos eventos). */
    private static String field(String text) {
        return text == null ? "" : text.replace('\t', ' ').replace('\n', ' ').replace('\r', ' ');
    }

    private static String message(Exception e) {
        String m = e.getMessage();
        return field(m == null || m.isEmpty() ? e.getClass().getSimpleName() : m);
    }

    public boolean nfcAvailable() {
        return android.nfc.NfcAdapter.getDefaultAdapter(this) != null;
    }

    public boolean nfcEnabled() {
        android.nfc.NfcAdapter a = android.nfc.NfcAdapter.getDefaultAdapter(this);
        return a != null && a.isEnabled();
    }

    public void nfcRead(boolean on) {
        nfcReading = on;
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                applyNfc();
            }
        });
    }

    /** Grava na próxima tag que encostar (a mensagem NDEF montada pelo aylis, em hex). */
    public void nfcWrite(String ndefHex) {
        nfcPendingWrite = ndefHex.isEmpty() ? null : unhex(ndefHex);
    }

    public String nfcPoll() {
        StringBuilder sb = new StringBuilder();
        String e;
        while ((e = nfcEvents.poll()) != null) {
            sb.append(e).append('\n');
        }
        return sb.toString();
    }

    private void applyNfc() {
        android.nfc.NfcAdapter a = android.nfc.NfcAdapter.getDefaultAdapter(this);
        if (a == null) {
            return;
        }
        try {
            if (nfcReading && resumed) {
                int flags = android.nfc.NfcAdapter.FLAG_READER_NFC_A | android.nfc.NfcAdapter.FLAG_READER_NFC_B | android.nfc.NfcAdapter.FLAG_READER_NFC_F
                    | android.nfc.NfcAdapter.FLAG_READER_NFC_V | android.nfc.NfcAdapter.FLAG_READER_NFC_BARCODE;
                a.enableReaderMode(this, new android.nfc.NfcAdapter.ReaderCallback() {
                    @Override
                    public void onTagDiscovered(android.nfc.Tag tag) {
                        onNfcTag(tag);
                    }
                }, flags, null);
            } else {
                a.disableReaderMode(this);
            }
        } catch (Exception e) {
            nfcEvents.add("error\t" + message(e));
        }
    }

    /** A tecnologia principal da tag, pelo nome curto (NfcA, MifareUltralight, IsoDep...). */
    private static String techName(android.nfc.Tag tag) {
        String best = "";
        for (String t : tag.getTechList()) {
            String name = t.substring(t.lastIndexOf('.') + 1);
            if (best.isEmpty() || name.startsWith("Mifare") || name.equals("IsoDep")) {
                best = name;
            }
        }
        return best;
    }

    private void onNfcTag(android.nfc.Tag tag) {
        android.nfc.tech.Ndef ndef = android.nfc.tech.Ndef.get(tag);
        byte[] pending = nfcPendingWrite;
        if (pending != null) {
            nfcPendingWrite = null;
            try {
                android.nfc.NdefMessage m = new android.nfc.NdefMessage(pending);
                if (ndef != null) {
                    ndef.connect();
                    try {
                        if (!ndef.isWritable()) {
                            throw new java.io.IOException("a tag é só de leitura");
                        }
                        if (ndef.getMaxSize() < pending.length) {
                            throw new java.io.IOException("a mensagem não cabe na tag (" + pending.length + " de " + ndef.getMaxSize() + " bytes)");
                        }
                        ndef.writeNdefMessage(m);
                    } finally {
                        ndef.close();
                    }
                } else {
                    android.nfc.tech.NdefFormatable f = android.nfc.tech.NdefFormatable.get(tag);
                    if (f == null) {
                        throw new java.io.IOException("a tag não aceita NDEF");
                    }
                    f.connect();
                    try {
                        f.format(m);
                    } finally {
                        f.close();
                    }
                    ndef = android.nfc.tech.Ndef.get(tag);
                }
                nfcEvents.add("written");
            } catch (Exception e) {
                nfcEvents.add("error\t" + message(e));
            }
        }
        byte[] message = null;
        boolean writable = false;
        if (ndef != null) {
            try {
                ndef.connect();
                try {
                    android.nfc.NdefMessage m = ndef.getNdefMessage();
                    if (m != null) {
                        message = m.toByteArray();
                    }
                    writable = ndef.isWritable();
                } finally {
                    ndef.close();
                }
            } catch (Exception e) {
                // Tirada antes de terminar: fica com o que tinha (o id, ao menos).
                android.nfc.NdefMessage m = ndef.getCachedNdefMessage();
                if (m != null) {
                    message = m.toByteArray();
                }
            }
        } else {
            writable = android.nfc.tech.NdefFormatable.get(tag) != null;
        }
        nfcEvents.add("tag\t" + hex(tag.getId()) + "\t" + techName(tag) + "\t" + (writable ? "1" : "0") + "\t" + hex(message));
    }

    @Override
    protected void onResume() {
        super.onResume();
        resumed = true;
        if (nfcReading) {
            applyNfc();
        }
    }

    @Override
    protected void onPause() {
        resumed = false;
        if (nfcReading) {
            applyNfc();
        }
        super.onPause();
    }

    // ----- Bluetooth: BLE (procurar, conectar, ler, gravar, notify) e o clássico (RFCOMM) -----
    // Os eventos vão para uma fila que o aylis lê (btPoll), um por linha, campos com tab:
    // "found id nome rssi", "scanstop", "connected id serviço/característica props;...",
    // "disconnected id", "value id característica hex", "written id característica",
    // "received id hex" e "error mensagem".
    private final java.util.concurrent.ConcurrentLinkedQueue<String> btEvents = new java.util.concurrent.ConcurrentLinkedQueue<String>();
    private final HashMap<String, android.bluetooth.BluetoothGatt> gatts = new HashMap<String, android.bluetooth.BluetoothGatt>();
    private final HashMap<String, android.bluetooth.BluetoothSocket> sockets = new HashMap<String, android.bluetooth.BluetoothSocket>();
    private android.bluetooth.le.ScanCallback btScanCallback;
    /** O Android faz uma operação GATT por vez: as outras esperam aqui. */
    private final java.util.ArrayDeque<Runnable> btOps = new java.util.ArrayDeque<Runnable>();
    private boolean btBusy;
    private static final java.util.UUID CCCD = java.util.UUID.fromString("00002902-0000-1000-8000-00805f9b34fb");
    private static final java.util.UUID SPP = java.util.UUID.fromString("00001101-0000-1000-8000-00805f9b34fb");

    private android.bluetooth.BluetoothAdapter btAdapter() {
        android.bluetooth.BluetoothManager m = (android.bluetooth.BluetoothManager) getSystemService(Context.BLUETOOTH_SERVICE);
        return m == null ? null : m.getAdapter();
    }

    public boolean btAvailable() {
        return btAdapter() != null;
    }

    public boolean btEnabled() {
        android.bluetooth.BluetoothAdapter a = btAdapter();
        return a != null && a.isEnabled();
    }

    /** As permissões de procurar e conectar (Android 12+) ou a localização (antes): pede as que faltam. */
    private boolean btPermissions() {
        String[] needed = Build.VERSION.SDK_INT >= 31
            ? new String[] { "android.permission.BLUETOOTH_SCAN", "android.permission.BLUETOOTH_CONNECT" }
            : new String[] { "android.permission.ACCESS_FINE_LOCATION" };
        boolean ok = true;
        for (String p : needed) {
            if (!ensurePermission(p)) {
                ok = false;
            }
        }
        if (!ok) {
            btEvents.add("error\tsem a permissão do Bluetooth (pedida agora; tente de novo)");
        }
        return ok;
    }

    private android.bluetooth.BluetoothAdapter btReady() {
        android.bluetooth.BluetoothAdapter a = btAdapter();
        if (a == null) {
            btEvents.add("error\teste aparelho não tem Bluetooth");
            return null;
        }
        if (!btPermissions()) {
            return null;
        }
        if (!a.isEnabled()) {
            btEvents.add("error\to Bluetooth está desligado");
            return null;
        }
        return a;
    }

    public String btPoll() {
        StringBuilder sb = new StringBuilder();
        String e;
        while ((e = btEvents.poll()) != null) {
            sb.append(e).append('\n');
        }
        return sb.toString();
    }

    public void btScan(final boolean on, final String services, final String name) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.bluetooth.BluetoothAdapter a = on ? btReady() : btAdapter();
                android.bluetooth.le.BluetoothLeScanner s = a == null ? null : a.getBluetoothLeScanner();
                try {
                    if (s != null && btScanCallback != null) {
                        s.stopScan(btScanCallback);
                    }
                    btScanCallback = null;
                    if (!on || s == null) {
                        btEvents.add("scanstop");
                        return;
                    }
                    java.util.ArrayList<android.bluetooth.le.ScanFilter> filters = new java.util.ArrayList<android.bluetooth.le.ScanFilter>();
                    for (String u : services.split(";")) {
                        if (!u.trim().isEmpty()) {
                            filters.add(new android.bluetooth.le.ScanFilter.Builder().setServiceUuid(new android.os.ParcelUuid(java.util.UUID.fromString(u.trim()))).build());
                        }
                    }
                    final String wanted = name.toLowerCase();
                    btScanCallback = new android.bluetooth.le.ScanCallback() {
                        @Override
                        public void onScanResult(int type, android.bluetooth.le.ScanResult r) {
                            String n = null;
                            try {
                                n = r.getDevice().getName();
                            } catch (SecurityException e) {
                                // Sem BLUETOOTH_CONNECT o nome do aparelho não vem; o do anúncio, sim.
                            }
                            if (n == null && r.getScanRecord() != null) {
                                n = r.getScanRecord().getDeviceName();
                            }
                            if (n == null) {
                                n = "";
                            }
                            if (!wanted.isEmpty() && !n.toLowerCase().contains(wanted)) {
                                return;
                            }
                            btEvents.add("found\t" + r.getDevice().getAddress() + "\t" + field(n) + "\t" + r.getRssi());
                        }

                        @Override
                        public void onScanFailed(int code) {
                            btEvents.add("error\tfalha ao procurar aparelhos (" + code + ")");
                            btEvents.add("scanstop");
                        }
                    };
                    android.bluetooth.le.ScanSettings settings = new android.bluetooth.le.ScanSettings.Builder().setScanMode(android.bluetooth.le.ScanSettings.SCAN_MODE_LOW_LATENCY).build();
                    s.startScan(filters, settings, btScanCallback);
                } catch (Exception e) {
                    btEvents.add("error\t" + message(e));
                    btEvents.add("scanstop");
                }
            }
        });
    }

    private static String btProps(int p) {
        StringBuilder sb = new StringBuilder();
        if ((p & android.bluetooth.BluetoothGattCharacteristic.PROPERTY_READ) != 0) sb.append("read,");
        if ((p & android.bluetooth.BluetoothGattCharacteristic.PROPERTY_WRITE) != 0) sb.append("write,");
        if ((p & android.bluetooth.BluetoothGattCharacteristic.PROPERTY_WRITE_NO_RESPONSE) != 0) sb.append("write_without_response,");
        if ((p & android.bluetooth.BluetoothGattCharacteristic.PROPERTY_NOTIFY) != 0) sb.append("notify,");
        if ((p & android.bluetooth.BluetoothGattCharacteristic.PROPERTY_INDICATE) != 0) sb.append("indicate,");
        return sb.length() > 0 ? sb.substring(0, sb.length() - 1) : "";
    }

    /** Põe a operação na fila (ou roda já, se nenhuma estiver em andamento). */
    private void btQueue(Runnable op) {
        synchronized (btOps) {
            btOps.add(op);
            if (btBusy) {
                return;
            }
            btBusy = true;
        }
        btNext();
    }

    /** Roda a próxima operação; cada uma chama btDone quando o Android responde. */
    private void btNext() {
        Runnable op;
        synchronized (btOps) {
            op = btOps.poll();
            if (op == null) {
                btBusy = false;
                return;
            }
        }
        try {
            op.run();
        } catch (Exception e) {
            btEvents.add("error\t" + message(e));
            btNext();
        }
    }

    private void btDone() {
        btNext();
    }

    public void btConnect(final String id) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.bluetooth.BluetoothAdapter a = btReady();
                if (a == null) {
                    return;
                }
                try {
                    android.bluetooth.BluetoothDevice d = a.getRemoteDevice(id);
                    android.bluetooth.BluetoothGatt g = d.connectGatt(AylisActivity.this, false, new android.bluetooth.BluetoothGattCallback() {
                        @Override
                        public void onConnectionStateChange(android.bluetooth.BluetoothGatt gatt, int status, int state) {
                            if (state == android.bluetooth.BluetoothProfile.STATE_CONNECTED) {
                                gatt.discoverServices();
                            } else if (state == android.bluetooth.BluetoothProfile.STATE_DISCONNECTED) {
                                synchronized (gatts) {
                                    gatts.remove(id);
                                }
                                gatt.close();
                                if (status != android.bluetooth.BluetoothGatt.GATT_SUCCESS) {
                                    btEvents.add("error\tconexão com " + id + " perdida (" + status + ")");
                                }
                                btEvents.add("disconnected\t" + id);
                            }
                        }

                        @Override
                        public void onServicesDiscovered(android.bluetooth.BluetoothGatt gatt, int status) {
                            StringBuilder sb = new StringBuilder();
                            for (android.bluetooth.BluetoothGattService svc : gatt.getServices()) {
                                for (android.bluetooth.BluetoothGattCharacteristic c : svc.getCharacteristics()) {
                                    sb.append(svc.getUuid()).append('/').append(c.getUuid()).append(' ').append(btProps(c.getProperties())).append(';');
                                }
                            }
                            btEvents.add("connected\t" + id + "\t" + sb);
                        }

                        @Override
                        @SuppressWarnings("deprecation")
                        public void onCharacteristicRead(android.bluetooth.BluetoothGatt gatt, android.bluetooth.BluetoothGattCharacteristic c, int status) {
                            if (status == android.bluetooth.BluetoothGatt.GATT_SUCCESS) {
                                btEvents.add("value\t" + id + "\t" + c.getUuid() + "\t" + hex(c.getValue()));
                            } else {
                                btEvents.add("error\tfalha ao ler " + c.getUuid() + " (" + status + ")");
                            }
                            btDone();
                        }

                        @Override
                        @SuppressWarnings("deprecation")
                        public void onCharacteristicChanged(android.bluetooth.BluetoothGatt gatt, android.bluetooth.BluetoothGattCharacteristic c) {
                            btEvents.add("value\t" + id + "\t" + c.getUuid() + "\t" + hex(c.getValue()));
                        }

                        @Override
                        public void onCharacteristicWrite(android.bluetooth.BluetoothGatt gatt, android.bluetooth.BluetoothGattCharacteristic c, int status) {
                            if (status == android.bluetooth.BluetoothGatt.GATT_SUCCESS) {
                                btEvents.add("written\t" + id + "\t" + c.getUuid());
                            } else {
                                btEvents.add("error\tfalha ao gravar " + c.getUuid() + " (" + status + ")");
                            }
                            btDone();
                        }

                        @Override
                        public void onDescriptorWrite(android.bluetooth.BluetoothGatt gatt, android.bluetooth.BluetoothGattDescriptor d, int status) {
                            if (status != android.bluetooth.BluetoothGatt.GATT_SUCCESS) {
                                btEvents.add("error\tfalha ao ligar o aviso de " + d.getCharacteristic().getUuid() + " (" + status + ")");
                            }
                            btDone();
                        }
                    }, android.bluetooth.BluetoothDevice.TRANSPORT_LE);
                    synchronized (gatts) {
                        gatts.put(id, g);
                    }
                } catch (Exception e) {
                    btEvents.add("error\t" + message(e));
                }
            }
        });
    }

    public void btDisconnect(String id) {
        android.bluetooth.BluetoothGatt g;
        synchronized (gatts) {
            g = gatts.get(id);
        }
        if (g != null) {
            try {
                g.disconnect();
            } catch (Exception e) {
                btEvents.add("error\t" + message(e));
            }
        }
    }

    private android.bluetooth.BluetoothGattCharacteristic btCharacteristic(android.bluetooth.BluetoothGatt g, String service, String characteristic) {
        android.bluetooth.BluetoothGattService s = g.getService(java.util.UUID.fromString(service));
        android.bluetooth.BluetoothGattCharacteristic c = s == null ? null : s.getCharacteristic(java.util.UUID.fromString(characteristic));
        if (c == null) {
            btEvents.add("error\tcaracterística " + characteristic + " não encontrada");
        }
        return c;
    }

    private android.bluetooth.BluetoothGatt btGatt(String id) {
        android.bluetooth.BluetoothGatt g;
        synchronized (gatts) {
            g = gatts.get(id);
        }
        if (g == null) {
            btEvents.add("error\t" + id + " não está conectado");
        }
        return g;
    }

    public void btRead(final String id, final String service, final String characteristic) {
        btQueue(new Runnable() {
            @Override
            public void run() {
                android.bluetooth.BluetoothGatt g = btGatt(id);
                android.bluetooth.BluetoothGattCharacteristic c = g == null ? null : btCharacteristic(g, service, characteristic);
                if (c == null || !g.readCharacteristic(c)) {
                    if (c != null) {
                        btEvents.add("error\tfalha ao ler " + characteristic);
                    }
                    btDone();
                }
            }
        });
    }

    @SuppressWarnings("deprecation")
    public void btWrite(final String id, final String service, final String characteristic, final String dataHex, final boolean withResponse) {
        btQueue(new Runnable() {
            @Override
            public void run() {
                android.bluetooth.BluetoothGatt g = btGatt(id);
                android.bluetooth.BluetoothGattCharacteristic c = g == null ? null : btCharacteristic(g, service, characteristic);
                if (c == null) {
                    btDone();
                    return;
                }
                c.setWriteType(withResponse ? android.bluetooth.BluetoothGattCharacteristic.WRITE_TYPE_DEFAULT : android.bluetooth.BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE);
                c.setValue(unhex(dataHex));
                if (!g.writeCharacteristic(c)) {
                    btEvents.add("error\tfalha ao gravar " + characteristic);
                    btDone();
                }
            }
        });
    }

    @SuppressWarnings("deprecation")
    public void btSubscribe(final String id, final String service, final String characteristic, final boolean on) {
        btQueue(new Runnable() {
            @Override
            public void run() {
                android.bluetooth.BluetoothGatt g = btGatt(id);
                android.bluetooth.BluetoothGattCharacteristic c = g == null ? null : btCharacteristic(g, service, characteristic);
                if (c == null) {
                    btDone();
                    return;
                }
                g.setCharacteristicNotification(c, on);
                android.bluetooth.BluetoothGattDescriptor d = c.getDescriptor(CCCD);
                if (d == null) {
                    btDone();
                    return;
                }
                boolean indicate = (c.getProperties() & android.bluetooth.BluetoothGattCharacteristic.PROPERTY_INDICATE) != 0
                    && (c.getProperties() & android.bluetooth.BluetoothGattCharacteristic.PROPERTY_NOTIFY) == 0;
                d.setValue(!on ? android.bluetooth.BluetoothGattDescriptor.DISABLE_NOTIFICATION_VALUE
                    : indicate ? android.bluetooth.BluetoothGattDescriptor.ENABLE_INDICATION_VALUE : android.bluetooth.BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE);
                if (!g.writeDescriptor(d)) {
                    btEvents.add("error\tfalha ao ligar o aviso de " + characteristic);
                    btDone();
                }
            }
        });
    }

    /** Os pareados: "id;nome" por linha. */
    public String btPaired() {
        android.bluetooth.BluetoothAdapter a = btAdapter();
        if (a == null || !btPermissions()) {
            return "";
        }
        StringBuilder sb = new StringBuilder();
        try {
            for (android.bluetooth.BluetoothDevice d : a.getBondedDevices()) {
                sb.append(d.getAddress()).append(';').append(field(d.getName())).append('\n');
            }
        } catch (SecurityException e) {
            btEvents.add("error\t" + message(e));
        }
        return sb.toString();
    }

    /** Abre o canal serial (SPP/RFCOMM) com um pareado, numa thread; o que chega vira "received". */
    public void btOpen(final String id) {
        final android.bluetooth.BluetoothAdapter a = btReady();
        if (a == null) {
            return;
        }
        new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    android.bluetooth.BluetoothDevice d = a.getRemoteDevice(id);
                    a.cancelDiscovery();
                    android.bluetooth.BluetoothSocket s = d.createRfcommSocketToServiceRecord(SPP);
                    s.connect();
                    synchronized (sockets) {
                        sockets.put(id, s);
                    }
                    btEvents.add("connected\t" + id + "\t");
                    java.io.InputStream in = s.getInputStream();
                    byte[] buffer = new byte[1024];
                    int n;
                    while ((n = in.read(buffer)) > 0) {
                        byte[] got = new byte[n];
                        System.arraycopy(buffer, 0, got, 0, n);
                        btEvents.add("received\t" + id + "\t" + hex(got));
                    }
                } catch (Exception e) {
                    btEvents.add("error\t" + message(e));
                }
                synchronized (sockets) {
                    sockets.remove(id);
                }
                btEvents.add("disconnected\t" + id);
            }
        }, "aylis-bt-" + id).start();
    }

    public boolean btSend(final String id, final String dataHex) {
        final android.bluetooth.BluetoothSocket s;
        synchronized (sockets) {
            s = sockets.get(id);
        }
        if (s == null) {
            return false;
        }
        new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    s.getOutputStream().write(unhex(dataHex));
                } catch (Exception e) {
                    btEvents.add("error\t" + message(e));
                }
            }
        }, "aylis-bt-send").start();
        return true;
    }

    public void btClose(String id) {
        android.bluetooth.BluetoothSocket s;
        synchronized (sockets) {
            s = sockets.remove(id);
        }
        if (s != null) {
            try {
                s.close();
            } catch (Exception e) {
                btEvents.add("error\t" + message(e));
            }
        }
    }

    // ----- Phone e LockScreen: discar, ligar, o estado das chamadas, o app de telefone -----
    private volatile String callState = "idle";
    private volatile String callNumber = "";
    private Object callWatcher;

    public boolean dial(String number) {
        try {
            startActivity(new android.content.Intent(android.content.Intent.ACTION_DIAL, android.net.Uri.parse("tel:" + android.net.Uri.encode(number))));
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    /** Liga direto; sem a permissão, pede e devolve false (o app tenta de novo). */
    public boolean call(String number) {
        if (!hasPermission("android.permission.CALL_PHONE")) {
            ensurePermission("android.permission.CALL_PHONE");
            return false;
        }
        try {
            startActivity(new android.content.Intent(android.content.Intent.ACTION_CALL, android.net.Uri.parse("tel:" + android.net.Uri.encode(number))));
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    private static String telephonyState(int state) {
        return state == android.telephony.TelephonyManager.CALL_STATE_RINGING ? "ringing"
            : state == android.telephony.TelephonyManager.CALL_STATE_OFFHOOK ? "active" : "idle";
    }

    /** O Android 12+ avisa o estado pelo TelephonyCallback. */
    private static class CallCallback extends android.telephony.TelephonyCallback implements android.telephony.TelephonyCallback.CallStateListener {
        @Override
        public void onCallStateChanged(int state) {
            AylisActivity a = current;
            if (a != null) {
                a.callState = telephonyState(state);
            }
        }
    }

    public void watchCalls(final boolean on) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.telephony.TelephonyManager tm = (android.telephony.TelephonyManager) getSystemService(Context.TELEPHONY_SERVICE);
                if (tm == null || !on || callWatcher != null) {
                    return;
                }
                if (!hasPermission("android.permission.READ_PHONE_STATE")) {
                    ensurePermission("android.permission.READ_PHONE_STATE");
                    return;
                }
                if (Build.VERSION.SDK_INT >= 31) {
                    CallCallback cb = new CallCallback();
                    tm.registerTelephonyCallback(getMainExecutor(), cb);
                    callWatcher = cb;
                } else {
                    android.telephony.PhoneStateListener l = new android.telephony.PhoneStateListener() {
                        @Override
                        public void onCallStateChanged(int state, String number) {
                            callState = telephonyState(state);
                            if (number != null && !number.isEmpty()) {
                                callNumber = number;
                            }
                        }
                    };
                    tm.listen(l, android.telephony.PhoneStateListener.LISTEN_CALL_STATE);
                    callWatcher = l;
                }
            }
        });
    }

    /** "estado;número": o do app de telefone (se for o discador) ou o do sistema. */
    public String callInfo() {
        if (callWatcher == null && hasPermission("android.permission.READ_PHONE_STATE")) {
            watchCalls(true);
        }
        android.telecom.Call c = Calls.current;
        if (c != null) {
            return Calls.stateName(c.getState()) + ";" + Calls.number;
        }
        return callState + ";" + callNumber;
    }

    public void requestDialer() {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.app.role.RoleManager rm = (android.app.role.RoleManager) getSystemService(Context.ROLE_SERVICE);
                if (rm != null && rm.isRoleAvailable(android.app.role.RoleManager.ROLE_DIALER) && !rm.isRoleHeld(android.app.role.RoleManager.ROLE_DIALER)) {
                    startActivityForResult(rm.createRequestRoleIntent(android.app.role.RoleManager.ROLE_DIALER), 7301);
                }
            }
        });
    }

    public boolean answer() {
        android.telecom.Call c = Calls.current;
        if (c == null) {
            return false;
        }
        c.answer(android.telecom.VideoProfile.STATE_AUDIO_ONLY);
        return true;
    }

    public boolean reject() {
        android.telecom.Call c = Calls.current;
        if (c == null) {
            return false;
        }
        c.reject(false, null);
        return true;
    }

    public boolean hangUp() {
        android.telecom.Call c = Calls.current;
        if (c == null) {
            return false;
        }
        c.disconnect();
        return true;
    }

    /** As últimas chamadas: "número|tipo|quando (ms)|segundos" por linha; null sem a permissão. */
    public String callHistory(int max) {
        if (!hasPermission("android.permission.READ_CALL_LOG")) {
            ensurePermission("android.permission.READ_CALL_LOG");
            return null;
        }
        StringBuilder out = new StringBuilder();
        try (android.database.Cursor c = getContentResolver().query(android.provider.CallLog.Calls.CONTENT_URI,
                new String[] { android.provider.CallLog.Calls.NUMBER, android.provider.CallLog.Calls.TYPE, android.provider.CallLog.Calls.DATE, android.provider.CallLog.Calls.DURATION },
                null, null, android.provider.CallLog.Calls.DATE + " DESC")) {
            int n = 0;
            while (c != null && c.moveToNext() && n++ < max) {
                int type = c.getInt(1);
                String kind = type == android.provider.CallLog.Calls.INCOMING_TYPE ? "recebida" : type == android.provider.CallLog.Calls.OUTGOING_TYPE ? "feita" : type == android.provider.CallLog.Calls.MISSED_TYPE ? "perdida" : "outra";
                out.append(c.getString(0)).append('|').append(kind).append('|').append(c.getLong(2)).append('|').append(c.getLong(3)).append('\n');
            }
        } catch (Exception e) {
            return null;
        }
        return out.toString();
    }

    /** O form por cima da tela de bloqueio e acender a tela ao abrir. */
    public void setLockScreen(final boolean show, final boolean turnOn) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                setShowWhenLocked(show);
                setTurnScreenOn(turnOn);
            }
        });
    }

    public void dismissKeyguard() {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.app.KeyguardManager km = (android.app.KeyguardManager) getSystemService(Context.KEYGUARD_SERVICE);
                if (km != null) {
                    km.requestDismissKeyguard(AylisActivity.this, null);
                }
            }
        });
    }

    /**
     * O modo "app de telefone": com o app como discador padrão, o sistema entrega as chamadas
     * aqui (atender, recusar e desligar ficam com o Phone) e abre o app a cada uma.
     */
    public static class Calls extends android.telecom.InCallService {
        static volatile android.telecom.Call current;
        static volatile String number = "";

        static String stateName(int state) {
            switch (state) {
                case android.telecom.Call.STATE_RINGING: return "ringing";
                case android.telecom.Call.STATE_DIALING:
                case android.telecom.Call.STATE_CONNECTING: return "dialing";
                case android.telecom.Call.STATE_ACTIVE: return "active";
                case android.telecom.Call.STATE_HOLDING: return "holding";
                default: return "idle";
            }
        }

        @Override
        public void onCallAdded(android.telecom.Call call) {
            current = call;
            android.net.Uri handle = call.getDetails().getHandle();
            number = handle == null ? "" : handle.getSchemeSpecificPart();
            android.content.Intent open = new android.content.Intent(this, AylisActivity.class);
            open.setFlags(android.content.Intent.FLAG_ACTIVITY_NEW_TASK);
            startActivity(open);
        }

        @Override
        public void onCallRemoved(android.telecom.Call call) {
            if (current == call) {
                current = null;
            }
        }
    }

    // ----- BackgroundService: o serviço em primeiro plano com a lógica sem tela do app -----
    static final String BACKGROUND = "aylis.background";

    /** Liga o serviço (a notificação fixa); com keepAlive, pede para sair da otimização de bateria. */
    public void startBackground(String title, String text, String kind, boolean keepAlive) {
        getSharedPreferences(BACKGROUND, MODE_PRIVATE).edit()
            .putString("title", title).putString("text", text).putString("kind", kind)
            .putBoolean("keep", keepAlive).putBoolean("on", true).apply();
        startForegroundService(new android.content.Intent(this, Background.class));
        android.os.PowerManager pm = (android.os.PowerManager) getSystemService(Context.POWER_SERVICE);
        if (keepAlive && pm != null && !pm.isIgnoringBatteryOptimizations(getPackageName())) {
            try {
                android.content.Intent ask = new android.content.Intent(android.provider.Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS);
                ask.setData(android.net.Uri.parse("package:" + getPackageName()));
                startActivity(ask);
            } catch (Exception e) {
                // Sem a tela do sistema: fica com a otimização.
            }
        }
    }

    public void stopBackground() {
        getSharedPreferences(BACKGROUND, MODE_PRIVATE).edit().putBoolean("on", false).apply();
        stopService(new android.content.Intent(this, Background.class));
    }

    /**
     * O serviço em primeiro plano: mostra a notificação fixa e roda a lógica sem tela do app
     * (nativeStart, que a geração do aylis exporta do lib.rs). Sobe também sozinho, sem a tela
     * (no boot ou quando o sistema o reinicia).
     */
    public static class Background extends android.app.Service {
        static native void nativeStart();

        @Override
        public android.os.IBinder onBind(android.content.Intent intent) {
            return null;
        }

        private int type(String kind) {
            switch (kind == null ? "" : kind) {
                case "location": return android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_LOCATION;
                case "mediaPlayback": return android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK;
                case "connectedDevice": return android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_CONNECTED_DEVICE;
                case "microphone": return android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_MICROPHONE;
                case "camera": return android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_CAMERA;
                default: return android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_DATA_SYNC;
            }
        }

        @Override
        public int onStartCommand(android.content.Intent intent, int flags, int startId) {
            android.content.SharedPreferences p = getSharedPreferences(BACKGROUND, MODE_PRIVATE);
            android.app.NotificationManager nm = (android.app.NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
            nm.createNotificationChannel(new android.app.NotificationChannel(BACKGROUND, "Segundo plano", android.app.NotificationManager.IMPORTANCE_LOW));
            android.content.Intent open = getPackageManager().getLaunchIntentForPackage(getPackageName());
            android.app.PendingIntent tap = open == null ? null : android.app.PendingIntent.getActivity(this, 7201, open, android.app.PendingIntent.FLAG_IMMUTABLE);
            int icon = getApplicationInfo().icon != 0 ? getApplicationInfo().icon : android.R.drawable.stat_notify_sync;
            android.app.Notification n = new android.app.Notification.Builder(this, BACKGROUND)
                .setContentTitle(p.getString("title", ""))
                .setContentText(p.getString("text", ""))
                .setSmallIcon(icon)
                .setOngoing(true)
                .setContentIntent(tap)
                .build();
            try {
                if (android.os.Build.VERSION.SDK_INT >= 29) {
                    startForeground(7201, n, type(p.getString("kind", "")));
                } else {
                    startForeground(7201, n);
                }
            } catch (Exception e) {
                // O tipo pede uma permissão que o app não tem: segue sem o serviço.
                stopSelf();
                return START_NOT_STICKY;
            }
            try {
                android.content.pm.ActivityInfo info = getPackageManager().getActivityInfo(
                    new android.content.ComponentName(this, AylisActivity.class), android.content.pm.PackageManager.GET_META_DATA);
                System.loadLibrary(info.metaData.getString("android.app.lib_name"));
                nativeStart();
            } catch (Throwable e) {
                // O app não tem a lógica sem tela (src/background.rs).
            }
            return p.getBoolean("keep", false) ? START_STICKY : START_NOT_STICKY;
        }
    }

    /** O boot: com keep_alive, o serviço volta sem a tela. */
    public static class Boot extends android.content.BroadcastReceiver {
        @Override
        public void onReceive(Context context, android.content.Intent intent) {
            android.content.SharedPreferences p = context.getSharedPreferences(BACKGROUND, Context.MODE_PRIVATE);
            if (p.getBoolean("on", false) && p.getBoolean("keep", false)) {
                context.startForegroundService(new android.content.Intent(context, Background.class));
            }
        }
    }

    // ----- Sensores: 0 localização, 1 bateria, 2 rede, 3 movimento, 4 bússola, 5 proximidade, 6 luz -----
    private final double[][] readings = new double[7][];
    private final android.hardware.SensorEventListener[] listeners = new android.hardware.SensorEventListener[7];
    private android.location.LocationListener locationListener;

    private int sensorType(int kind) {
        switch (kind) {
            case 3: return android.hardware.Sensor.TYPE_ACCELEROMETER;
            case 4: return android.hardware.Sensor.TYPE_ROTATION_VECTOR;
            case 5: return android.hardware.Sensor.TYPE_PROXIMITY;
            case 6: return android.hardware.Sensor.TYPE_LIGHT;
            default: return -1;
        }
    }

    public boolean sensorAvailable(int kind) {
        if (kind == 0) {
            return getSystemService(Context.LOCATION_SERVICE) != null;
        }
        if (kind == 1 || kind == 2) {
            return true;
        }
        android.hardware.SensorManager sm = (android.hardware.SensorManager) getSystemService(Context.SENSOR_SERVICE);
        return sm != null && sm.getDefaultSensor(sensorType(kind)) != null;
    }

    /** Liga ou desliga a medição (a localização pede a permissão na primeira vez). */
    public void sensorActive(final int kind, final boolean on) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                if (kind == 0) {
                    locationActive(on);
                    return;
                }
                final android.hardware.SensorManager sm = (android.hardware.SensorManager) getSystemService(Context.SENSOR_SERVICE);
                if (sm == null || kind < 3) {
                    return;
                }
                if (listeners[kind] != null) {
                    sm.unregisterListener(listeners[kind]);
                    listeners[kind] = null;
                }
                if (!on) {
                    return;
                }
                final android.hardware.Sensor sensor = sm.getDefaultSensor(sensorType(kind));
                if (sensor == null) {
                    return;
                }
                listeners[kind] = new android.hardware.SensorEventListener() {
                    @Override
                    public void onSensorChanged(android.hardware.SensorEvent e) {
                        if (kind == 4) {
                            float[] m = new float[9];
                            float[] o = new float[3];
                            android.hardware.SensorManager.getRotationMatrixFromVector(m, e.values);
                            android.hardware.SensorManager.getOrientation(m, o);
                            readings[kind] = new double[] { (Math.toDegrees(o[0]) + 360.0) % 360.0 };
                        } else if (kind == 5) {
                            readings[kind] = new double[] { e.values[0], e.values[0] < sensor.getMaximumRange() ? 1 : 0 };
                        } else if (kind == 3) {
                            readings[kind] = new double[] { e.values[0], e.values[1], e.values[2] };
                        } else {
                            readings[kind] = new double[] { e.values[0] };
                        }
                    }

                    @Override
                    public void onAccuracyChanged(android.hardware.Sensor s, int accuracy) {
                    }
                };
                sm.registerListener(listeners[kind], sensor, android.hardware.SensorManager.SENSOR_DELAY_UI);
            }
        });
    }

    private void locationActive(boolean on) {
        android.location.LocationManager lm = (android.location.LocationManager) getSystemService(Context.LOCATION_SERVICE);
        if (lm == null) {
            return;
        }
        if (locationListener != null) {
            lm.removeUpdates(locationListener);
            locationListener = null;
        }
        if (!on) {
            return;
        }
        if (!hasPermission("android.permission.ACCESS_FINE_LOCATION") && !hasPermission("android.permission.ACCESS_COARSE_LOCATION")) {
            // Pede e tenta de novo quando o aylis chamar outra vez (o componente religa).
            ensurePermission("android.permission.ACCESS_FINE_LOCATION");
            return;
        }
        locationListener = new android.location.LocationListener() {
            @Override
            public void onLocationChanged(android.location.Location l) {
                readings[0] = new double[] { l.getLatitude(), l.getLongitude(), l.getAccuracy(), l.getAltitude(), l.getSpeed() };
            }

            @Override
            public void onProviderEnabled(String provider) {
            }

            @Override
            public void onProviderDisabled(String provider) {
            }

            @Override
            public void onStatusChanged(String provider, int status, Bundle extras) {
            }
        };
        try {
            for (String provider : lm.getProviders(true)) {
                android.location.Location last = lm.getLastKnownLocation(provider);
                if (last != null && readings[0] == null) {
                    locationListener.onLocationChanged(last);
                }
                lm.requestLocationUpdates(provider, 1000, 0, locationListener, getMainLooper());
            }
        } catch (SecurityException e) {
            // Sem a permissão: fica sem posição.
        }
    }

    /** A última medida (bateria e rede: agora); null = ainda não mediu. */
    public double[] sensorRead(int kind) {
        if (kind == 0 && readings[0] == null && locationListener == null
            && (hasPermission("android.permission.ACCESS_FINE_LOCATION") || hasPermission("android.permission.ACCESS_COARSE_LOCATION"))) {
            // A permissão chegou depois de ligar: liga de novo.
            sensorActive(0, true);
        }
        if (kind == 1) {
            android.content.Intent b = registerReceiver(null, new android.content.IntentFilter(android.content.Intent.ACTION_BATTERY_CHANGED));
            if (b == null) {
                return null;
            }
            int level = b.getIntExtra(android.os.BatteryManager.EXTRA_LEVEL, -1);
            int scale = b.getIntExtra(android.os.BatteryManager.EXTRA_SCALE, 100);
            int status = b.getIntExtra(android.os.BatteryManager.EXTRA_STATUS, -1);
            boolean charging = status == android.os.BatteryManager.BATTERY_STATUS_CHARGING || status == android.os.BatteryManager.BATTERY_STATUS_FULL;
            return new double[] { level < 0 ? 0 : (double) level / scale, charging ? 1 : 0 };
        }
        if (kind == 2) {
            android.net.ConnectivityManager cm = (android.net.ConnectivityManager) getSystemService(Context.CONNECTIVITY_SERVICE);
            android.net.NetworkCapabilities caps = cm == null ? null : cm.getNetworkCapabilities(cm.getActiveNetwork());
            if (caps == null || !caps.hasCapability(android.net.NetworkCapabilities.NET_CAPABILITY_INTERNET)) {
                return new double[] { 0, 0 };
            }
            int type = caps.hasTransport(android.net.NetworkCapabilities.TRANSPORT_WIFI) ? 1
                : caps.hasTransport(android.net.NetworkCapabilities.TRANSPORT_CELLULAR) ? 2
                : caps.hasTransport(android.net.NetworkCapabilities.TRANSPORT_ETHERNET) ? 3 : 4;
            return new double[] { 1, type };
        }
        return kind >= 0 && kind < readings.length ? readings[kind] : null;
    }

    public boolean vibrate(int ms) {
        android.os.Vibrator v = (android.os.Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
        if (v == null || !v.hasVibrator()) {
            return false;
        }
        v.vibrate(android.os.VibrationEffect.createOneShot(ms, android.os.VibrationEffect.DEFAULT_AMPLITUDE));
        return true;
    }

    /** Os controles de jogo conectados, um por linha: "id\tnome" (o Gamepad do aylis). */
    public String gamepads() {
        StringBuilder out = new StringBuilder();
        for (int id : android.view.InputDevice.getDeviceIds()) {
            android.view.InputDevice d = android.view.InputDevice.getDevice(id);
            if (d == null || d.isVirtual()) {
                continue;
            }
            int s = d.getSources();
            boolean pad = (s & android.view.InputDevice.SOURCE_GAMEPAD) == android.view.InputDevice.SOURCE_GAMEPAD
                    || (s & android.view.InputDevice.SOURCE_JOYSTICK) == android.view.InputDevice.SOURCE_JOYSTICK;
            if (pad) {
                out.append(id).append('\t').append(d.getName().replace('\n', ' ').replace('\t', ' ')).append('\n');
            }
        }
        return out.toString();
    }

    /** Vibra o controle de jogo (amplitude de 1 a 255). */
    public boolean gamepadRumble(int id, int amplitude, int ms) {
        android.view.InputDevice d = android.view.InputDevice.getDevice(id);
        if (d == null) {
            return false;
        }
        android.os.Vibrator v = d.getVibrator();
        if (v == null || !v.hasVibrator()) {
            return false;
        }
        v.vibrate(android.os.VibrationEffect.createOneShot(Math.max(1, ms), Math.max(1, Math.min(255, amplitude))));
        return true;
    }

    /** Pede a ação a outro app; a resposta volta em nativeLinkResult. */
    public void request(final long request, final String action, final String data) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.content.Intent intent = new android.content.Intent(action);
                if (data != null && !data.isEmpty()) {
                    intent.setData(android.net.Uri.parse(data));
                }
                linkRequest = request;
                try {
                    startActivityForResult(intent, LINK_REQUEST);
                } catch (Exception e) {
                    nativeLinkResult(request, 0, "nenhum app faz " + action);
                }
            }
        });
    }

    /**
     * Os arquivos que o app compartilha (cache/shared), lidos por outro app pelo content://
     * (o FileProvider é do androidx, que o aylis não usa). Declarado no manifesto pela
     * publicação quando o app usa o AppLink.
     */
    public static class Files extends android.content.ContentProvider {
        static String mime(String name) {
            int dot = name == null ? -1 : name.lastIndexOf('.');
            String type = dot < 0 ? null : android.webkit.MimeTypeMap.getSingleton().getMimeTypeFromExtension(name.substring(dot + 1).toLowerCase());
            return type == null ? "application/octet-stream" : type;
        }

        private java.io.File file(android.net.Uri uri) {
            return new java.io.File(new java.io.File(getContext().getCacheDir(), "shared"), new java.io.File(uri.getLastPathSegment()).getName());
        }

        @Override
        public boolean onCreate() {
            return true;
        }

        @Override
        public android.os.ParcelFileDescriptor openFile(android.net.Uri uri, String mode) throws java.io.FileNotFoundException {
            return android.os.ParcelFileDescriptor.open(file(uri), android.os.ParcelFileDescriptor.MODE_READ_ONLY);
        }

        @Override
        public android.database.Cursor query(android.net.Uri uri, String[] projection, String selection, String[] args, String sort) {
            java.io.File f = file(uri);
            android.database.MatrixCursor c = new android.database.MatrixCursor(new String[] { android.provider.OpenableColumns.DISPLAY_NAME, android.provider.OpenableColumns.SIZE });
            c.addRow(new Object[] { f.getName(), f.length() });
            return c;
        }

        @Override
        public String getType(android.net.Uri uri) {
            return mime(uri.getLastPathSegment());
        }

        @Override
        public android.net.Uri insert(android.net.Uri uri, android.content.ContentValues values) {
            return null;
        }

        @Override
        public int delete(android.net.Uri uri, String selection, String[] args) {
            return 0;
        }

        @Override
        public int update(android.net.Uri uri, android.content.ContentValues values, String selection, String[] args) {
            return 0;
        }
    }

    @Override
    protected void onDestroy() {
        for (WebView w : webviews.values()) {
            getWindowManager().removeViewImmediate(w);
            w.destroy();
        }
        webviews.clear();
        super.onDestroy();
    }

    // ----- WebView (o componente WebView do aylis) -----
    // A superfície da janela é do código nativo (as views da Activity não aparecem): cada
    // WebView fica numa janela-filha (painel) sobre a da Activity, com superfície e toques
    // próprios. Todos chamados pelo aylis de qualquer thread; o trabalho vai para a thread
    // da tela. Posições em pixels da janela.

    private WindowManager.LayoutParams panel(int x, int y, int width, int height) {
        WindowManager.LayoutParams p = new WindowManager.LayoutParams(
                Math.max(1, width), Math.max(1, height), x, y,
                WindowManager.LayoutParams.TYPE_APPLICATION_PANEL,
                WindowManager.LayoutParams.FLAG_NOT_TOUCH_MODAL | WindowManager.LayoutParams.FLAG_LAYOUT_IN_SCREEN,
                PixelFormat.TRANSLUCENT);
        p.gravity = Gravity.TOP | Gravity.LEFT;
        p.token = getWindow().getDecorView().getWindowToken();
        p.softInputMode = WindowManager.LayoutParams.SOFT_INPUT_ADJUST_RESIZE;
        return p;
    }

    public void webCreate(final long id) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                if (webviews.containsKey(id)) {
                    return;
                }
                WebView w = new WebView(AylisActivity.this);
                WebSettings s = w.getSettings();
                s.setJavaScriptEnabled(true);
                s.setDomStorageEnabled(true);
                w.setWebViewClient(new WebViewClient() {
                    @Override
                    public boolean shouldOverrideUrlLoading(WebView view, WebResourceRequest request) {
                        nativeWebEvent(id, 1, request.getUrl().toString());
                        return false;
                    }

                    @Override
                    public void onPageFinished(WebView view, String url) {
                        nativeWebEvent(id, 0, url);
                    }
                });
                // window.ipc.postMessage("..."), como no desktop.
                w.addJavascriptInterface(new Object() {
                    @JavascriptInterface
                    public void postMessage(String text) {
                        nativeWebEvent(id, 2, text);
                    }
                }, "ipc");
                w.setVisibility(View.GONE);
                getWindowManager().addView(w, panel(0, 0, 1, 1));
                webviews.put(id, w);
            }
        });
    }

    public void webLoad(final long id, final String url, final String html) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                WebView w = webviews.get(id);
                if (w == null) {
                    return;
                }
                if (url.length() > 0) {
                    w.loadUrl(url);
                } else {
                    w.loadDataWithBaseURL(null, html, "text/html", "UTF-8", null);
                }
            }
        });
    }

    public void webBounds(final long id, final int x, final int y, final int width, final int height, final boolean visible) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                WebView w = webviews.get(id);
                if (w == null) {
                    return;
                }
                getWindowManager().updateViewLayout(w, panel(x, y, width, height));
                w.setVisibility(visible ? View.VISIBLE : View.GONE);
            }
        });
    }

    /** cmd: 0 = rodar JavaScript (text), 1 = voltar, 2 = avançar, 3 = recarregar, 4 = fechar. */
    public void webCommand(final long id, final int cmd, final String text) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                WebView w = webviews.get(id);
                if (w == null) {
                    return;
                }
                switch (cmd) {
                    case 0:
                        w.evaluateJavascript(text, null);
                        break;
                    case 1:
                        if (w.canGoBack()) w.goBack();
                        break;
                    case 2:
                        if (w.canGoForward()) w.goForward();
                        break;
                    case 3:
                        w.reload();
                        break;
                    default:
                        getWindowManager().removeViewImmediate(w);
                        w.destroy();
                        webviews.remove(id);
                }
            }
        });
    }

    /**
     * A activity em uso, para o código nativo que só tem o contexto do aplicativo (câmera,
     * microfone: o aylis-media pede as permissões por ela).
     */
    public static volatile AylisActivity current;

    /** A permissão (android.permission.CAMERA...) já foi concedida? */
    public boolean hasPermission(String name) {
        return checkSelfPermission(name) == android.content.pm.PackageManager.PERMISSION_GRANTED;
    }

    /** Pedidos de permissão na fila: o Android descarta um pedido feito com outro aberto. */
    private final java.util.ArrayDeque<String> permissionQueue = new java.util.ArrayDeque<String>();
    private String askingPermission;
    private final java.util.HashSet<String> answeredPermissions = new java.util.HashSet<String>();

    /**
     * Concedida: true. Senão põe o pedido na fila (a janela do sistema, um de cada vez) e
     * devolve false; o aylis consulta permissionState até a resposta.
     */
    public boolean ensurePermission(final String name) {
        if (hasPermission(name)) {
            return true;
        }
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                if (!name.equals(askingPermission) && !permissionQueue.contains(name)) {
                    answeredPermissions.remove(name);
                    permissionQueue.add(name);
                }
                askNextPermission();
            }
        });
        return false;
    }

    /** 1: concedida; 0: esperando o usuário; -1: negada. */
    public int permissionState(String name) {
        if (hasPermission(name)) {
            return 1;
        }
        synchronized (answeredPermissions) {
            return answeredPermissions.contains(name) ? -1 : 0;
        }
    }

    private void askNextPermission() {
        if (askingPermission != null || permissionQueue.isEmpty()) {
            return;
        }
        askingPermission = permissionQueue.poll();
        requestPermissions(new String[] { askingPermission }, 1);
    }

    @Override
    public void onRequestPermissionsResult(int code, String[] permissions, int[] results) {
        super.onRequestPermissionsResult(code, permissions, results);
        synchronized (answeredPermissions) {
            if (askingPermission != null) {
                answeredPermissions.add(askingPermission);
            }
        }
        askingPermission = null;
        // Notificações que esperavam a permissão (Android 13+).
        if (hasPermission("android.permission.POST_NOTIFICATIONS") || Build.VERSION.SDK_INT < 33) {
            for (Runnable r : waitingNotifications) {
                r.run();
            }
        }
        if (permissionState("android.permission.POST_NOTIFICATIONS") != 0) {
            waitingNotifications.clear();
        }
        askNextPermission();
    }

    // ----- arquivos (OpenDialog, SaveDialog, FileInput) -----
    // O seletor de documentos do sistema. Ao abrir, cada documento escolhido é copiado para
    // o cache do app (o aylis lê como arquivo comum); ao salvar, volta o URI content:// e o
    // aylis grava nele por writeUri.

    private static final int PICK_OPEN = 7101;
    private static final int PICK_SAVE = 7102;
    private long pickRequest;

    /** Abre o seletor. mimeTypes vazio = qualquer arquivo. */
    public void openDocument(final long request, final boolean multiple, final String[] mimeTypes) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.content.Intent intent = new android.content.Intent(android.content.Intent.ACTION_OPEN_DOCUMENT);
                intent.addCategory(android.content.Intent.CATEGORY_OPENABLE);
                intent.setType("*/*");
                if (mimeTypes != null && mimeTypes.length > 0) {
                    intent.putExtra(android.content.Intent.EXTRA_MIME_TYPES, mimeTypes);
                }
                intent.putExtra(android.content.Intent.EXTRA_ALLOW_MULTIPLE, multiple);
                pickRequest = request;
                try {
                    startActivityForResult(intent, PICK_OPEN);
                } catch (Exception e) {
                    nativeFilesPicked(request, "");
                }
            }
        });
    }

    /** Pergunta onde salvar (o nome sugerido): volta o URI do documento criado. */
    public void createDocument(final long request, final String name, final String mimeType) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.content.Intent intent = new android.content.Intent(android.content.Intent.ACTION_CREATE_DOCUMENT);
                intent.addCategory(android.content.Intent.CATEGORY_OPENABLE);
                intent.setType(mimeType == null || mimeType.isEmpty() ? "application/octet-stream" : mimeType);
                intent.putExtra(android.content.Intent.EXTRA_TITLE, name);
                pickRequest = request;
                try {
                    startActivityForResult(intent, PICK_SAVE);
                } catch (Exception e) {
                    nativeFilesPicked(request, "");
                }
            }
        });
    }

    @Override
    protected void onActivityResult(int code, int result, android.content.Intent data) {
        super.onActivityResult(code, result, data);
        if (code == LOOPBACK_REQUEST) {
            if (result != RESULT_OK || data == null) {
                loopbackState = 3;
                return;
            }
            projectionResult = result;
            projectionData = data;
            startForegroundService(new android.content.Intent(this, Projection.class));
            return;
        }
        if (code == LINK_REQUEST) {
            String text = data == null ? "" : data.getDataString() != null ? data.getDataString() : data.getExtras() != null ? data.getExtras().toString() : "";
            nativeLinkResult(linkRequest, result == RESULT_OK ? 1 : 0, text);
            return;
        }
        if (code != PICK_OPEN && code != PICK_SAVE) {
            return;
        }
        final long request = pickRequest;
        if (result != RESULT_OK || data == null) {
            nativeFilesPicked(request, "");
            return;
        }
        final java.util.ArrayList<android.net.Uri> uris = new java.util.ArrayList<android.net.Uri>();
        if (data.getClipData() != null) {
            for (int i = 0; i < data.getClipData().getItemCount(); i++) {
                uris.add(data.getClipData().getItemAt(i).getUri());
            }
        } else if (data.getData() != null) {
            uris.add(data.getData());
        }
        if (code == PICK_SAVE) {
            nativeFilesPicked(request, uris.isEmpty() ? "" : uris.get(0).toString());
            return;
        }
        // Copiar pode demorar (arquivos grandes, na nuvem): fora da thread da tela.
        new Thread(new Runnable() {
            @Override
            public void run() {
                StringBuilder paths = new StringBuilder();
                java.io.File dir = new java.io.File(getCacheDir(), "picked/" + System.currentTimeMillis());
                dir.mkdirs();
                for (android.net.Uri uri : uris) {
                    String name = displayName(uri);
                    java.io.File out = new java.io.File(dir, name);
                    try (java.io.InputStream in = getContentResolver().openInputStream(uri);
                         java.io.OutputStream os = new java.io.FileOutputStream(out)) {
                        byte[] buf = new byte[1 << 16];
                        int n;
                        while ((n = in.read(buf)) > 0) {
                            os.write(buf, 0, n);
                        }
                        if (paths.length() > 0) {
                            paths.append('\n');
                        }
                        paths.append(out.getAbsolutePath());
                    } catch (Exception e) {
                        // O que não deu para copiar fica de fora.
                    }
                }
                nativeFilesPicked(request, paths.toString());
            }
        }).start();
    }

    /** O nome do documento (o que o usuário vê no seletor). */
    private String displayName(android.net.Uri uri) {
        String name = null;
        try (android.database.Cursor c = getContentResolver().query(uri, new String[] { android.provider.OpenableColumns.DISPLAY_NAME }, null, null, null)) {
            if (c != null && c.moveToFirst()) {
                name = c.getString(0);
            }
        } catch (Exception e) {
            // sem nome: usa o fim do URI
        }
        if (name == null || name.isEmpty()) {
            name = uri.getLastPathSegment();
        }
        return name == null ? "arquivo" : name.replace('/', '_');
    }

    /** Grava num documento content:// (o que o SaveDialog devolveu). */
    public boolean writeUri(String uri, byte[] data) {
        try (java.io.OutputStream os = getContentResolver().openOutputStream(android.net.Uri.parse(uri), "wt")) {
            os.write(data);
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    /** Lê um documento content://. */
    public byte[] readUri(String uri) {
        try (java.io.InputStream in = getContentResolver().openInputStream(android.net.Uri.parse(uri));
             java.io.ByteArrayOutputStream out = new java.io.ByteArrayOutputStream()) {
            byte[] buf = new byte[1 << 16];
            int n;
            while ((n = in.read(buf)) > 0) {
                out.write(buf, 0, n);
            }
            return out.toByteArray();
        } catch (Exception e) {
            return null;
        }
    }

    // ----- biometria (o componente Biometrics) -----

    /**
     * 0 nenhuma; 1 digital; 2 rosto; 3 íris; 4 tem, sem saber qual. Pelo BiometricManager
     * (Android 10+) ou, antes, pelos recursos do aparelho.
     */
    public int biometricKind() {
        if (Build.VERSION.SDK_INT < 28) {
            return 0;
        }
        if (Build.VERSION.SDK_INT >= 29) {
            android.hardware.biometrics.BiometricManager m = getSystemService(android.hardware.biometrics.BiometricManager.class);
            int ok = Build.VERSION.SDK_INT >= 30
                ? m.canAuthenticate(android.hardware.biometrics.BiometricManager.Authenticators.BIOMETRIC_WEAK)
                : m.canAuthenticate();
            if (ok != android.hardware.biometrics.BiometricManager.BIOMETRIC_SUCCESS) {
                return 0;
            }
        }
        android.content.pm.PackageManager pm = getPackageManager();
        int kinds = 0;
        int kind = 4;
        if (pm.hasSystemFeature(android.content.pm.PackageManager.FEATURE_FINGERPRINT)) { kinds++; kind = 1; }
        if (Build.VERSION.SDK_INT >= 29 && pm.hasSystemFeature(android.content.pm.PackageManager.FEATURE_FACE)) { kinds++; kind = 2; }
        if (Build.VERSION.SDK_INT >= 29 && pm.hasSystemFeature(android.content.pm.PackageManager.FEATURE_IRIS)) { kinds++; kind = 3; }
        return kinds == 1 ? kind : 4;
    }

    /** Mostra a verificação do sistema; a resposta volta por nativeBiometricResult. */
    public void authenticate(final long request, final String title, final boolean allowDeviceCredential) {
        if (Build.VERSION.SDK_INT < 28) {
            nativeBiometricResult(request, 2, "biometria exige Android 9 ou mais novo");
            return;
        }
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                android.hardware.biometrics.BiometricPrompt.Builder b = new android.hardware.biometrics.BiometricPrompt.Builder(AylisActivity.this).setTitle(title);
                final android.os.CancellationSignal cancel = new android.os.CancellationSignal();
                if (allowDeviceCredential && Build.VERSION.SDK_INT >= 30) {
                    b.setAllowedAuthenticators(android.hardware.biometrics.BiometricManager.Authenticators.BIOMETRIC_WEAK
                        | android.hardware.biometrics.BiometricManager.Authenticators.DEVICE_CREDENTIAL);
                } else if (allowDeviceCredential && Build.VERSION.SDK_INT >= 29) {
                    b.setDeviceCredentialAllowed(true);
                } else {
                    b.setNegativeButton(getString(android.R.string.cancel), getMainExecutor(), new android.content.DialogInterface.OnClickListener() {
                        @Override
                        public void onClick(android.content.DialogInterface d, int which) {
                            nativeBiometricResult(request, 1, "");
                        }
                    });
                }
                try {
                    b.build().authenticate(cancel, getMainExecutor(), new android.hardware.biometrics.BiometricPrompt.AuthenticationCallback() {
                        @Override
                        public void onAuthenticationSucceeded(android.hardware.biometrics.BiometricPrompt.AuthenticationResult r) {
                            nativeBiometricResult(request, 0, "");
                        }

                        @Override
                        public void onAuthenticationError(int code, CharSequence message) {
                            boolean cancelled = code == android.hardware.biometrics.BiometricPrompt.BIOMETRIC_ERROR_USER_CANCELED
                                || code == android.hardware.biometrics.BiometricPrompt.BIOMETRIC_ERROR_CANCELED;
                            nativeBiometricResult(request, cancelled ? 1 : 2, message == null ? "" : message.toString());
                        }
                    });
                } catch (Exception e) {
                    nativeBiometricResult(request, 2, String.valueOf(e.getMessage()));
                }
            }
        });
    }

    // ----- PushNotifications: o FCM pelo Google Play Services, sem o SDK do Firebase -----
    // O registro faz o que o SDK faz: a instalação no Firebase Installations (HTTP) e o
    // pedido do token ao Play Services (o mesmo canal Messenger do SDK). As mensagens chegam
    // pelo broadcast c2dm.intent.RECEIVE (o receiver Push). Os eventos vão para uma fila que
    // o aylis lê (pushPoll), um por linha: "token t", "message id título texto dados",
    // "opened ..." e "error mensagem" (campos com tab).
    static final String PUSH = "aylis.push";
    static final java.util.concurrent.ConcurrentLinkedQueue<String> pushEvents = new java.util.concurrent.ConcurrentLinkedQueue<String>();
    private static final String GMS = "com.google.android.gms";
    private static final String FIS = "https://firebaseinstallations.googleapis.com/v1/projects/";
    private static final java.util.concurrent.atomic.AtomicInteger pushRequest = new java.util.concurrent.atomic.AtomicInteger(1);

    public boolean pushAvailable() {
        try {
            getPackageManager().getPackageInfo(GMS, 0);
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    public String pushPoll() {
        StringBuilder sb = new StringBuilder();
        String e;
        while ((e = pushEvents.poll()) != null) {
            sb.append(e).append('\n');
        }
        return sb.toString();
    }

    /** O toque na notificação push (a linha da mensagem vai no extra "aylis.push"). */
    private boolean pushOpened(android.content.Intent intent) {
        String line = intent == null ? null : intent.getStringExtra(PUSH);
        if (line == null) {
            return false;
        }
        intent.removeExtra(PUSH);
        pushEvents.add("opened\t" + line);
        return true;
    }

    public void pushRegister(final String sender, final String project, final String appId, final String apiKey, final boolean foreground) {
        getSharedPreferences(PUSH, MODE_PRIVATE).edit().putBoolean("foreground", foreground).apply();
        if (sender.isEmpty() || project.isEmpty() || appId.isEmpty() || apiKey.isEmpty()) {
            pushEvents.add("error\tFCM: preencha sender_id, project_id, app_id e api_key (do google-services.json)");
            return;
        }
        if (Build.VERSION.SDK_INT >= 33) {
            ensurePermission("android.permission.POST_NOTIFICATIONS");
        }
        new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    String[] installation = fisInstallation(project, appId, apiKey);
                    android.os.Bundle reply = gmsRpc(gmsRequest(sender, appId, installation[0], installation[1]));
                    String token = reply.getString("registration_id");
                    if (token == null) {
                        String error = reply.getString("error");
                        throw new java.io.IOException("FCM: " + (error == null ? "o Play Services não deu o token" : error));
                    }
                    getSharedPreferences(PUSH, MODE_PRIVATE).edit().putString("token", token).apply();
                    pushEvents.add("token\t" + field(token));
                } catch (Exception e) {
                    pushEvents.add("error\t" + message(e));
                }
            }
        }).start();
    }

    public void pushUnregister() {
        final android.content.SharedPreferences p = getSharedPreferences(PUSH, MODE_PRIVATE);
        new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    String fid = p.getString("fid", null);
                    if (fid != null) {
                        android.os.Bundle data = gmsRequest(p.getString("sender", ""), p.getString("config", ""), fid, p.getString("auth", ""));
                        data.putString("delete", "1");
                        gmsRpc(data);
                    }
                } catch (Exception e) {
                    // Sem rede: o token some no servidor quando o envio falhar.
                }
                p.edit().remove("token").apply();
            }
        }).start();
    }

    /** O id da instalação e o token de autorização (cria a instalação na primeira vez). */
    private String[] fisInstallation(String project, String appId, String apiKey) throws Exception {
        android.content.SharedPreferences p = getSharedPreferences(PUSH, MODE_PRIVATE);
        if (!appId.equals(p.getString("config", ""))) {
            p.edit().remove("fid").remove("refresh").remove("token").apply();
        }
        String fid = p.getString("fid", null);
        String refresh = p.getString("refresh", null);
        if (fid != null && refresh != null) {
            try {
                org.json.JSONObject r = fisPost(FIS + project + "/installations/" + fid + "/authTokens:generate", apiKey, "FIS_v2 " + refresh,
                    "{\"installation\":{\"sdkVersion\":\"a:18.0.0\"}}");
                String auth = r.getString("token");
                p.edit().putString("auth", auth).apply();
                return new String[] { fid, auth };
            } catch (Exception e) {
                // A instalação foi apagada no servidor: cria outra.
            }
        }
        if (fid == null) {
            byte[] b = new byte[17];
            new java.security.SecureRandom().nextBytes(b);
            b[16] = b[0];
            b[0] = (byte) (0x70 | (b[0] & 0x0F));
            fid = android.util.Base64.encodeToString(b, android.util.Base64.URL_SAFE | android.util.Base64.NO_WRAP | android.util.Base64.NO_PADDING).substring(0, 22);
        }
        org.json.JSONObject body = new org.json.JSONObject();
        body.put("fid", fid).put("appId", appId).put("authVersion", "FIS_v2").put("sdkVersion", "a:18.0.0");
        org.json.JSONObject r = fisPost(FIS + project + "/installations", apiKey, null, body.toString());
        fid = r.optString("fid", fid);
        String auth = r.getJSONObject("authToken").getString("token");
        p.edit().putString("fid", fid).putString("refresh", r.getString("refreshToken")).putString("auth", auth).putString("config", appId).apply();
        return new String[] { fid, auth };
    }

    private org.json.JSONObject fisPost(String url, String apiKey, String authorization, String body) throws Exception {
        java.net.HttpURLConnection c = (java.net.HttpURLConnection) new java.net.URL(url).openConnection();
        c.setRequestMethod("POST");
        c.setDoOutput(true);
        c.setConnectTimeout(15000);
        c.setReadTimeout(15000);
        c.setRequestProperty("Content-Type", "application/json");
        c.setRequestProperty("Accept", "application/json");
        c.setRequestProperty("x-goog-api-key", apiKey);
        c.setRequestProperty("X-Android-Package", getPackageName());
        c.setRequestProperty("X-Android-Cert", signatureSha1());
        if (authorization != null) {
            c.setRequestProperty("Authorization", authorization);
        }
        try (java.io.OutputStream os = c.getOutputStream()) {
            os.write(body.getBytes("UTF-8"));
        }
        int code = c.getResponseCode();
        java.io.InputStream in = code < 300 ? c.getInputStream() : c.getErrorStream();
        StringBuilder sb = new StringBuilder();
        if (in != null) {
            try (java.io.BufferedReader r = new java.io.BufferedReader(new java.io.InputStreamReader(in, "UTF-8"))) {
                String line;
                while ((line = r.readLine()) != null) {
                    sb.append(line);
                }
            }
        }
        if (code >= 300) {
            String detail = sb.toString();
            try {
                detail = new org.json.JSONObject(detail).getJSONObject("error").getString("message");
            } catch (Exception e) {
                // O corpo como veio.
            }
            throw new java.io.IOException("Firebase Installations " + code + ": " + detail);
        }
        return new org.json.JSONObject(sb.toString());
    }

    /** O SHA-1 do certificado que assinou o app (o Firebase confere com a chave da API). */
    private String signatureSha1() {
        try {
            byte[] cert;
            if (Build.VERSION.SDK_INT >= 28) {
                android.content.pm.PackageInfo info = getPackageManager().getPackageInfo(getPackageName(), android.content.pm.PackageManager.GET_SIGNING_CERTIFICATES);
                cert = info.signingInfo.getApkContentsSigners()[0].toByteArray();
            } else {
                android.content.pm.PackageInfo info = getPackageManager().getPackageInfo(getPackageName(), android.content.pm.PackageManager.GET_SIGNATURES);
                cert = info.signatures[0].toByteArray();
            }
            return hex(java.security.MessageDigest.getInstance("SHA1").digest(cert));
        } catch (Exception e) {
            return "";
        }
    }

    /** O pedido do token, com os campos que o SDK do Firebase manda. */
    private android.os.Bundle gmsRequest(String sender, String appId, String fid, String auth) throws Exception {
        getSharedPreferences(PUSH, MODE_PRIVATE).edit().putString("sender", sender).apply();
        android.content.pm.PackageInfo self = getPackageManager().getPackageInfo(getPackageName(), 0);
        android.os.Bundle data = new android.os.Bundle();
        data.putString("scope", "*");
        data.putString("sender", sender);
        data.putString("subtype", sender);
        data.putString("gmp_app_id", appId);
        data.putString("gmsv", String.valueOf(getPackageManager().getPackageInfo(GMS, 0).versionCode));
        data.putString("osv", String.valueOf(Build.VERSION.SDK_INT));
        data.putString("app_ver", String.valueOf(self.versionCode));
        data.putString("app_ver_name", self.versionName == null ? "" : self.versionName);
        byte[] hash = java.security.MessageDigest.getInstance("SHA-1").digest("[DEFAULT]".getBytes("UTF-8"));
        data.putString("firebase-app-name-hash", android.util.Base64.encodeToString(hash, android.util.Base64.URL_SAFE | android.util.Base64.NO_WRAP | android.util.Base64.NO_PADDING));
        data.putString("appid", fid);
        data.putString("Goog-Firebase-Installations-Auth", auth);
        data.putString("cliv", "fcm-24.0.0");
        return data;
    }

    /** Manda o pedido ao Play Services (o serviço Messenger do c2dm) e espera a resposta. */
    private android.os.Bundle gmsRpc(final android.os.Bundle data) throws Exception {
        final java.util.concurrent.ArrayBlockingQueue<android.os.Bundle> reply = new java.util.concurrent.ArrayBlockingQueue<android.os.Bundle>(1);
        final android.os.HandlerThread thread = new android.os.HandlerThread(PUSH);
        thread.start();
        final int id = pushRequest.getAndIncrement();
        final android.os.Messenger messenger = new android.os.Messenger(new android.os.Handler(thread.getLooper()) {
            @Override
            public void handleMessage(android.os.Message m) {
                android.os.Bundle b = m.getData();
                if (b.getBoolean("unsupported", false)) {
                    android.os.Bundle e = new android.os.Bundle();
                    e.putString("error", "o Play Services não aceita este pedido (atualize o Google Play Services)");
                    reply.offer(e);
                } else {
                    android.os.Bundle d = b.getBundle("data");
                    reply.offer(d == null ? new android.os.Bundle() : d);
                }
            }
        });
        android.content.ServiceConnection connection = new android.content.ServiceConnection() {
            @Override
            public void onServiceConnected(android.content.ComponentName name, android.os.IBinder binder) {
                try {
                    android.os.Message m = android.os.Message.obtain();
                    m.what = 1;
                    m.arg1 = id;
                    m.replyTo = messenger;
                    android.os.Bundle b = new android.os.Bundle();
                    b.putBoolean("oneWay", false);
                    b.putString("pkg", getPackageName());
                    b.putBundle("data", data);
                    m.setData(b);
                    new android.os.Messenger(binder).send(m);
                } catch (Exception e) {
                    android.os.Bundle err = new android.os.Bundle();
                    err.putString("error", message(e));
                    reply.offer(err);
                }
            }

            @Override
            public void onServiceDisconnected(android.content.ComponentName name) {
            }
        };
        android.content.Intent intent = new android.content.Intent("com.google.android.c2dm.intent.REGISTER").setPackage(GMS);
        try {
            if (!bindService(intent, connection, Context.BIND_AUTO_CREATE)) {
                throw new java.io.IOException("o Google Play Services não está disponível");
            }
            android.os.Bundle r = reply.poll(30, java.util.concurrent.TimeUnit.SECONDS);
            if (r == null) {
                throw new java.io.IOException("o Google Play Services não respondeu");
            }
            return r;
        } finally {
            try {
                unbindService(connection);
            } catch (Exception e) {
                // Não chegou a ligar.
            }
            thread.quitSafely();
        }
    }

    /** As mensagens do FCM (o Play Services entrega por broadcast, mesmo com o app fechado). */
    public static class Push extends android.content.BroadcastReceiver {
        @Override
        public void onReceive(Context context, android.content.Intent intent) {
            if (!"com.google.android.c2dm.intent.RECEIVE".equals(intent.getAction()) || intent.getExtras() == null) {
                return;
            }
            android.os.Bundle x = intent.getExtras();
            String type = x.getString("message_type");
            if (type != null && !"gcm".equals(type)) {
                return;
            }
            String id = x.getString("google.message_id", x.getString("message_id", ""));
            String title = notificationField(x, "title");
            String body = notificationField(x, "body");
            org.json.JSONObject data = new org.json.JSONObject();
            for (String key : x.keySet()) {
                if (key.startsWith("google.") || key.startsWith("gcm.") || key.equals("from") || key.equals("collapse_key") || key.equals("message_type")
                        || key.equals("message_id")) {
                    continue;
                }
                Object value = x.get(key);
                try {
                    data.put(key, value == null ? "" : value.toString());
                } catch (Exception e) {
                    // Chave inválida: fica de fora.
                }
            }
            final String line = field(id) + "\t" + field(title) + "\t" + field(body) + "\t" + data.toString();
            pushEvents.add("message\t" + line);
            // Os dados com o título, o texto e o id, para os filtros e os modelos das regras
            // (uma chave dos dados com o mesmo nome vale mais).
            try {
                if (!data.has("title")) data.put("title", title);
                if (!data.has("body")) data.put("body", body);
                if (!data.has("message_id")) data.put("message_id", id);
            } catch (Exception e) {
                // Fica sem eles.
            }
            final Context app = context.getApplicationContext();
            final String ruleData = data.toString();
            final String fId = id, fTitle = title, fBody = body;
            // As regras do PushTrigger podem baixar uma imagem: fora da thread principal.
            final PendingResult pending = goAsync();
            new Thread(new Runnable() {
                @Override
                public void run() {
                    try {
                        boolean replaced = runPushRules(app, ruleData);
                        AylisActivity a = current;
                        boolean foreground = a != null && a.resumed;
                        boolean show = !foreground || app.getSharedPreferences(PUSH, Context.MODE_PRIVATE).getBoolean("foreground", true);
                        if (!replaced && show && (!fTitle.isEmpty() || !fBody.isEmpty())) {
                            showPush(app, fId, fTitle, fBody, line);
                        }
                    } finally {
                        pending.finish();
                    }
                }
            }).start();
        }

        private static String notificationField(android.os.Bundle x, String name) {
            String v = x.getString("gcm.notification." + name);
            if (v == null) {
                v = x.getString("gcm.n." + name);
            }
            return v == null ? "" : v;
        }

        private static void showPush(Context context, String id, String title, String body, String line) {
            android.app.NotificationManager nm = (android.app.NotificationManager) context.getSystemService(Context.NOTIFICATION_SERVICE);
            nm.createNotificationChannel(new android.app.NotificationChannel(PUSH, "Notificações", android.app.NotificationManager.IMPORTANCE_HIGH));
            int n = (id.isEmpty() ? String.valueOf(System.currentTimeMillis()) : id).hashCode() & 0x7fffffff;
            android.content.Intent open = new android.content.Intent(context, AylisActivity.class);
            open.setFlags(android.content.Intent.FLAG_ACTIVITY_NEW_TASK | android.content.Intent.FLAG_ACTIVITY_SINGLE_TOP | android.content.Intent.FLAG_ACTIVITY_CLEAR_TOP);
            open.setAction("aylis.push." + n);
            open.putExtra(PUSH, line);
            android.app.PendingIntent pi = android.app.PendingIntent.getActivity(context, n, open,
                android.app.PendingIntent.FLAG_UPDATE_CURRENT | android.app.PendingIntent.FLAG_IMMUTABLE);
            int icon = context.getApplicationInfo().icon != 0 ? context.getApplicationInfo().icon : android.R.drawable.ic_dialog_info;
            android.app.Notification.Builder b = new android.app.Notification.Builder(context, PUSH)
                .setSmallIcon(icon)
                .setContentTitle(title)
                .setContentText(body)
                .setAutoCancel(true)
                .setContentIntent(pi);
            if (body.length() > 40) {
                b.setStyle(new android.app.Notification.BigTextStyle().bigText(body));
            }
            nm.notify(PUSH, n, b.build());
        }
    }

    // ----- Ações do aparelho (WakeAppAction, NotifyAction) e as regras do PushTrigger -----
    // A ação chega como JSON (o spec que o aylis monta). O que acontece com ela (o app abriu,
    // um botão, o tempo acabou, uma regra disparou) fica guardado nas preferências, para
    // valer também com o app fechado, até o aylis ler (actionsPoll): uma linha por evento,
    // "tipo componente detalhe dados" separados por tab.
    static final String ACTIONS = "aylis.actions";

    /** Uma linha só (os dados JSON podem vir com quebras: são espaços no JSON). */
    private static String oneLine(String text) {
        return text == null ? "" : text.replace('\n', ' ').replace('\r', ' ').replace('\t', ' ');
    }

    static void actionEvent(Context c, String kind, String component, String detail, String data) {
        synchronized (ACTIONS) {
            android.content.SharedPreferences p = c.getSharedPreferences(ACTIONS, Context.MODE_PRIVATE);
            String line = kind + "\t" + field(component) + "\t" + field(detail) + "\t" + oneLine(data) + "\n";
            p.edit().putString("pending", p.getString("pending", "") + line).commit();
        }
    }

    public String actionsPoll() {
        synchronized (ACTIONS) {
            android.content.SharedPreferences p = getSharedPreferences(ACTIONS, MODE_PRIVATE);
            String pending = p.getString("pending", "");
            if (!pending.isEmpty()) {
                p.edit().remove("pending").commit();
            }
            return pending;
        }
    }

    /** As regras dos PushTrigger (uma lista JSON), gravadas para valer com o app fechado. */
    public void actionsRules(String rules) {
        getSharedPreferences(ACTIONS, MODE_PRIVATE).edit().putString("rules", rules).apply();
    }

    /** Executa a ação; devolve o erro (vazio: deu certo). */
    public String actionExecute(final String spec, final String data) {
        try {
            final org.json.JSONObject s = new org.json.JSONObject(spec);
            final String d = data == null || data.trim().isEmpty() ? "{}" : data;
            if ("notify".equals(s.optString("type")) && s.optString("image").contains("http")) {
                // A imagem pela rede: fora da thread do aylis.
                new Thread(new Runnable() {
                    @Override
                    public void run() {
                        try {
                            runAction(AylisActivity.this, s, d);
                        } catch (Exception e) {
                            // Sem a notificação.
                        }
                    }
                }).start();
            } else {
                runAction(this, s, d);
            }
            if (Build.VERSION.SDK_INT >= 33) {
                ensurePermission("android.permission.POST_NOTIFICATIONS");
            }
            return "";
        } catch (Exception e) {
            return message(e);
        }
    }

    /** Para a ação: tira a notificação (e o toque) e deixa de mostrar o app por cima do bloqueio. */
    public void actionStop(String component) {
        android.app.NotificationManager nm = (android.app.NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
        nm.cancel(ACTIONS, actionId(component));
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                if (Build.VERSION.SDK_INT >= 27) {
                    setShowWhenLocked(false);
                    setTurnScreenOn(false);
                }
            }
        });
    }

    static int actionId(String component) {
        return component.hashCode() & 0x7fffffff;
    }

    /** O texto de modelo com os dados: {chave} vira o valor; {{ e }} são as chaves. */
    static String render(String template, org.json.JSONObject data) {
        StringBuilder out = new StringBuilder();
        int i = 0;
        while (i < template.length()) {
            char c = template.charAt(i);
            if ((c == '{' || c == '}') && i + 1 < template.length() && template.charAt(i + 1) == c) {
                out.append(c);
                i += 2;
            } else if (c == '{' && template.indexOf('}', i) > i) {
                int end = template.indexOf('}', i);
                Object v = data.opt(template.substring(i + 1, end).trim());
                out.append(v == null || v == org.json.JSONObject.NULL ? "" : v.toString());
                i = end + 1;
            } else {
                out.append(c);
                i++;
            }
        }
        return out.toString();
    }

    /** O filtro do PushTrigger: chave=valor, chave!=valor ou chave (existe), separados por ;. */
    static boolean matches(String filter, org.json.JSONObject data) {
        for (String raw : filter.split(";")) {
            String c = raw.trim();
            if (c.isEmpty()) {
                continue;
            }
            int ne = c.indexOf("!="), eq = c.indexOf('=');
            if (ne > 0) {
                if (data.optString(c.substring(0, ne).trim(), "").equals(c.substring(ne + 2).trim())) return false;
            } else if (eq > 0) {
                if (!data.optString(c.substring(0, eq).trim(), "").equals(c.substring(eq + 1).trim())) return false;
            } else if (data.optString(c, "").isEmpty()) {
                return false;
            }
        }
        return true;
    }

    /** As regras gravadas que casam com a mensagem: executa a ação e avisa o PushTrigger. Devolve se alguma pediu para não mostrar a notificação comum. */
    static boolean runPushRules(Context c, String data) {
        boolean replaced = false;
        try {
            org.json.JSONArray rules = new org.json.JSONArray(c.getSharedPreferences(ACTIONS, Context.MODE_PRIVATE).getString("rules", "[]"));
            org.json.JSONObject d = new org.json.JSONObject(data);
            for (int i = 0; i < rules.length(); i++) {
                org.json.JSONObject rule = rules.getJSONObject(i);
                if (!matches(rule.optString("filter"), d)) {
                    continue;
                }
                try {
                    runAction(c, rule.getJSONObject("action"), data);
                } catch (Exception e) {
                    // A regra seguinte ainda vale.
                }
                actionEvent(c, "trigger", rule.optString("trigger"), "", data);
                replaced |= rule.optBoolean("replace", true);
            }
        } catch (Exception e) {
            // Sem regras.
        }
        return replaced;
    }

    /** Uma intenção que abre o app pela ação (com o botão, se houver). */
    private static android.app.PendingIntent openIntent(Context c, String extra, String component, String button, String data, boolean wake, boolean screenOn, int request) {
        android.content.Intent open = new android.content.Intent(c, AylisActivity.class);
        open.setFlags(android.content.Intent.FLAG_ACTIVITY_NEW_TASK | android.content.Intent.FLAG_ACTIVITY_SINGLE_TOP | android.content.Intent.FLAG_ACTIVITY_CLEAR_TOP);
        open.setAction("aylis.action." + request);
        open.putExtra(extra, component);
        open.putExtra("aylis.action.data", data);
        open.putExtra("aylis.action.wake", wake);
        open.putExtra("aylis.action.screen", screenOn);
        if (button != null) {
            open.putExtra("aylis.action.button", button);
        }
        return android.app.PendingIntent.getActivity(c, request, open, android.app.PendingIntent.FLAG_UPDATE_CURRENT | android.app.PendingIntent.FLAG_IMMUTABLE);
    }

    /** Um botão que responde sem abrir o app (o receiver Actions), ou o fim do tempo. */
    private static android.app.PendingIntent backgroundIntent(Context c, String kind, String component, String button, String data, int request) {
        android.content.Intent i = new android.content.Intent(c, Actions.class);
        i.setAction("aylis.action." + kind + "." + request);
        i.putExtra("kind", kind);
        i.putExtra("component", component);
        i.putExtra("button", button == null ? "" : button);
        i.putExtra("data", data);
        return android.app.PendingIntent.getBroadcast(c, request, i, android.app.PendingIntent.FLAG_UPDATE_CURRENT | android.app.PendingIntent.FLAG_IMMUTABLE);
    }

    private static boolean opens(org.json.JSONArray open, String id) {
        for (int i = 0; i < open.length(); i++) {
            String o = open.optString(i);
            if ("*".equals(o) || id.equals(o)) {
                return true;
            }
        }
        return false;
    }

    /** O botão no estilo de chamada (answer, ou reject/decline). */
    private static android.app.PendingIntent findButton(java.util.List<String> ids, java.util.List<android.app.PendingIntent> intents, String... names) {
        for (String name : names) {
            int i = ids.indexOf(name);
            if (i >= 0) {
                return intents.get(i);
            }
        }
        return null;
    }

    static void runAction(Context c, org.json.JSONObject spec, String dataText) throws Exception {
        org.json.JSONObject data = new org.json.JSONObject(dataText);
        String component = spec.optString("component");
        boolean wake = "wake".equals(spec.optString("type"));
        int id = actionId(component);
        int base = (id % 100000) * 16;
        String title = render(spec.optString("title"), data);
        String text = render(spec.optString("text"), data);
        android.app.NotificationManager nm = (android.app.NotificationManager) c.getSystemService(Context.NOTIFICATION_SERVICE);
        int icon = c.getApplicationInfo().icon != 0 ? c.getApplicationInfo().icon : android.R.drawable.ic_dialog_info;
        String channel;
        if (wake) {
            String ringtone = spec.optString("ringtone", "default");
            boolean vibrate = spec.optBoolean("vibrate", true);
            channel = ACTIONS + ".wake." + ringtone + (vibrate ? ".v" : "");
            android.app.NotificationChannel ch = new android.app.NotificationChannel(channel, "Chamadas", android.app.NotificationManager.IMPORTANCE_HIGH);
            int type = "alarm".equals(ringtone) ? android.media.RingtoneManager.TYPE_ALARM
                : "notification".equals(ringtone) ? android.media.RingtoneManager.TYPE_NOTIFICATION : android.media.RingtoneManager.TYPE_RINGTONE;
            if ("none".equals(ringtone)) {
                ch.setSound(null, null);
            } else {
                android.media.AudioAttributes attrs = new android.media.AudioAttributes.Builder()
                    .setUsage("alarm".equals(ringtone) ? android.media.AudioAttributes.USAGE_ALARM : android.media.AudioAttributes.USAGE_NOTIFICATION_RINGTONE)
                    .setContentType(android.media.AudioAttributes.CONTENT_TYPE_SONIFICATION)
                    .build();
                ch.setSound(android.media.RingtoneManager.getDefaultUri(type), attrs);
            }
            ch.enableVibration(vibrate);
            if (vibrate) {
                ch.setVibrationPattern(new long[] { 0, 1000, 1000 });
            }
            ch.setLockscreenVisibility(android.app.Notification.VISIBILITY_PUBLIC);
            nm.createNotificationChannel(ch);
        } else {
            channel = spec.optString("channel", "geral");
            int priority = spec.optInt("priority", 3);
            nm.createNotificationChannel(new android.app.NotificationChannel(channel, spec.optString("channel_name", "Geral"), priority));
        }
        boolean screenOn = spec.optBoolean("screen_on", true);
        String extra = wake ? "aylis.wake" : "aylis.action.click";
        android.app.PendingIntent body = openIntent(c, extra, component, null, dataText, wake, screenOn, base);
        android.app.Notification.Builder b = new android.app.Notification.Builder(c, channel)
            .setSmallIcon(icon)
            .setContentTitle(title)
            .setContentText(text)
            .setContentIntent(body)
            .setVisibility(android.app.Notification.VISIBILITY_PUBLIC);
        // Os botões: os de "open" abrem o app; os outros respondem pelo receiver.
        org.json.JSONArray buttons = spec.optJSONArray("buttons");
        org.json.JSONArray open = spec.optJSONArray("open");
        if (open == null) {
            open = new org.json.JSONArray();
        }
        java.util.List<String> ids = new java.util.ArrayList<String>();
        java.util.List<String> labels = new java.util.ArrayList<String>();
        java.util.List<android.app.PendingIntent> intents = new java.util.ArrayList<android.app.PendingIntent>();
        for (int i = 0; buttons != null && i < buttons.length() && i < 6; i++) {
            org.json.JSONArray button = buttons.getJSONArray(i);
            String bid = button.optString(0);
            ids.add(bid);
            labels.add(render(button.optString(1, bid), data));
            intents.add(opens(open, bid) ? openIntent(c, extra, component, bid, dataText, wake, screenOn, base + 1 + i)
                : backgroundIntent(c, "button", component, bid, dataText, base + 1 + i));
        }
        boolean styled = false;
        if (wake) {
            b.setCategory(android.app.Notification.CATEGORY_CALL).setOngoing(true).setAutoCancel(false).setFullScreenIntent(body, true);
            double timeout = spec.optDouble("timeout", 0);
            if (timeout > 0) {
                // Sem resposta no tempo: o sistema tira a notificação e o delete avisa.
                b.setTimeoutAfter((long) (timeout * 1000));
                b.setDeleteIntent(backgroundIntent(c, "timeout", component, null, dataText, base + 15));
            }
            android.app.PendingIntent answer = findButton(ids, intents, "answer", "accept");
            android.app.PendingIntent decline = findButton(ids, intents, "reject", "decline", "hangup");
            if (spec.optBoolean("call_style") && Build.VERSION.SDK_INT >= 31 && answer != null && decline != null) {
                android.app.Person caller = new android.app.Person.Builder().setName(title.isEmpty() ? spec.optString("app_name", " ") : title).setImportant(true).build();
                b.setStyle(android.app.Notification.CallStyle.forIncomingCall(caller, decline, answer));
                styled = true;
            }
        } else {
            b.setAutoCancel(true);
            android.graphics.Bitmap image = null;
            String png = spec.optString("image_png");
            String url = render(spec.optString("image"), data);
            if (!png.isEmpty()) {
                byte[] bytes = android.util.Base64.decode(png, android.util.Base64.DEFAULT);
                image = android.graphics.BitmapFactory.decodeByteArray(bytes, 0, bytes.length);
            } else if (url.startsWith("http://") || url.startsWith("https://")) {
                try (java.io.InputStream in = new java.net.URL(url).openStream()) {
                    image = android.graphics.BitmapFactory.decodeStream(in);
                } catch (Exception e) {
                    // Sem a imagem.
                }
            }
            if (image != null) {
                b.setLargeIcon(image).setStyle(new android.app.Notification.BigPictureStyle().bigPicture(image));
            } else if (text.length() > 40) {
                b.setStyle(new android.app.Notification.BigTextStyle().bigText(text));
            }
        }
        for (int i = 0; i < ids.size(); i++) {
            String bid = ids.get(i);
            boolean inStyle = styled && ("answer".equals(bid) || "accept".equals(bid) || "reject".equals(bid) || "decline".equals(bid) || "hangup".equals(bid));
            if (!inStyle) {
                b.addAction(new android.app.Notification.Action.Builder(
                    android.graphics.drawable.Icon.createWithResource(c, android.R.drawable.ic_menu_send), labels.get(i), intents.get(i)).build());
            }
        }
        android.app.Notification n = b.build();
        if (wake && !"none".equals(spec.optString("ringtone", "default"))) {
            // O toque repete até a resposta (ou o fim do tempo).
            n.flags |= android.app.Notification.FLAG_INSISTENT;
        }
        nm.notify(ACTIONS, id, n);
    }

    /** O app aberto por uma ação: por cima do bloqueio (WakeAppAction) e os eventos. */
    private boolean actionOpened(android.content.Intent intent) {
        if (intent == null) {
            return false;
        }
        String wake = intent.getStringExtra("aylis.wake");
        String click = intent.getStringExtra("aylis.action.click");
        if (wake == null && click == null) {
            return false;
        }
        String data = intent.getStringExtra("aylis.action.data");
        String button = intent.getStringExtra("aylis.action.button");
        if (wake != null) {
            final boolean screenOn = intent.getBooleanExtra("aylis.action.screen", true);
            if (Build.VERSION.SDK_INT >= 27) {
                setShowWhenLocked(true);
                if (screenOn) {
                    setTurnScreenOn(true);
                }
            } else {
                getWindow().addFlags(WindowManager.LayoutParams.FLAG_SHOW_WHEN_LOCKED | (screenOn ? WindowManager.LayoutParams.FLAG_TURN_SCREEN_ON : 0));
            }
            actionEvent(this, "open", wake, "", data);
            if (button != null) {
                // Um botão que abre o app (atender): o toque para.
                ((android.app.NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE)).cancel(ACTIONS, actionId(wake));
                actionEvent(this, "button", wake, button, data);
            }
        } else {
            actionEvent(this, button != null ? "button" : "click", click, button == null ? "" : button, data);
        }
        intent.removeExtra("aylis.wake");
        intent.removeExtra("aylis.action.click");
        intent.removeExtra("aylis.action.button");
        return true;
    }

    /** Os botões que respondem sem abrir o app e o fim do tempo do WakeAppAction. */
    public static class Actions extends android.content.BroadcastReceiver {
        @Override
        public void onReceive(Context context, android.content.Intent intent) {
            String kind = intent.getStringExtra("kind");
            String component = intent.getStringExtra("component");
            if (kind == null || component == null) {
                return;
            }
            if ("button".equals(kind)) {
                ((android.app.NotificationManager) context.getSystemService(Context.NOTIFICATION_SERVICE)).cancel(ACTIONS, actionId(component));
            }
            actionEvent(context, kind, component, intent.getStringExtra("button"), intent.getStringExtra("data"));
        }
    }

    // ----- notificações -----

    private final java.util.ArrayList<Runnable> waitingNotifications = new java.util.ArrayList<Runnable>();

    /** Um toque na notificação (ou num botão dela) chega aqui com o app aberto. */
    @Override
    protected void onNewIntent(android.content.Intent intent) {
        super.onNewIntent(intent);
        if (pushOpened(intent) || actionOpened(intent)) {
            return;
        }
        if (intent != null && intent.hasExtra("aylis.component")) {
            String action = intent.getStringExtra("aylis.action");
            nativeNotification(intent.getLongExtra("aylis.component", 0), action == null ? "" : action);
        } else if (intent != null) {
            deliver(intent);
        }
    }

    private android.app.PendingIntent notificationIntent(int request, long component, String action) {
        android.content.Intent intent = new android.content.Intent(this, AylisActivity.class);
        intent.setFlags(android.content.Intent.FLAG_ACTIVITY_SINGLE_TOP | android.content.Intent.FLAG_ACTIVITY_CLEAR_TOP);
        intent.putExtra("aylis.component", component);
        intent.putExtra("aylis.action", action);
        intent.setAction("aylis.notification." + request);
        return android.app.PendingIntent.getActivity(this, request, intent,
            android.app.PendingIntent.FLAG_UPDATE_CURRENT | android.app.PendingIntent.FLAG_IMMUTABLE);
    }

    /**
     * Mostra (ou atualiza, pelo id) uma notificação. importance: 2 baixa, 3 padrão, 4 alta;
     * progress: -1 sem barra, 0 a 1000; png: imagem (vazia: nenhuma); big: imagem grande.
     */
    public void showNotification(final int id, final String channel, final String channelName, final int importance,
            final String title, final String text, final byte[] png, final boolean big,
            final String[] actionIds, final String[] actionTitles, final boolean ongoing, final int progress, final long component,
            final boolean fullScreen) {
        Runnable post = new Runnable() {
            @Override
            public void run() {
                android.app.NotificationManager manager = (android.app.NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
                manager.createNotificationChannel(new android.app.NotificationChannel(channel, channelName, importance));
                int icon = getApplicationInfo().icon != 0 ? getApplicationInfo().icon : android.R.drawable.ic_dialog_info;
                android.app.Notification.Builder b = new android.app.Notification.Builder(AylisActivity.this, channel)
                    .setSmallIcon(icon)
                    .setContentTitle(title)
                    .setContentText(text)
                    .setOngoing(ongoing)
                    .setAutoCancel(!ongoing)
                    .setContentIntent(notificationIntent(id * 16, component, ""));
                if (png != null && png.length > 0) {
                    android.graphics.Bitmap bmp = android.graphics.BitmapFactory.decodeByteArray(png, 0, png.length);
                    if (bmp != null) {
                        if (big) {
                            b.setStyle(new android.app.Notification.BigPictureStyle().bigPicture(bmp));
                        } else {
                            b.setLargeIcon(bmp);
                        }
                    }
                } else if (text != null && text.length() > 40) {
                    b.setStyle(new android.app.Notification.BigTextStyle().bigText(text));
                }
                if (progress >= 0) {
                    b.setProgress(1000, progress, false);
                }
                if (fullScreen) {
                    // Por cima de tudo (a tela de bloqueio também): uma ligação, um alarme.
                    android.app.PendingIntent full = notificationIntent(id * 16 + 15, component, "");
                    b.setFullScreenIntent(full, true).setCategory(android.app.Notification.CATEGORY_CALL);
                }
                for (int i = 0; i < actionIds.length && i < 3; i++) {
                    b.addAction(new android.app.Notification.Action.Builder(
                        android.graphics.drawable.Icon.createWithResource(AylisActivity.this, android.R.drawable.ic_menu_send),
                        actionTitles[i], notificationIntent(id * 16 + 1 + i, component, actionIds[i])).build());
                }
                manager.notify(id, b.build());
            }
        };
        if (Build.VERSION.SDK_INT >= 33 && !hasPermission("android.permission.POST_NOTIFICATIONS")) {
            runOnUiThread(new Runnable() {
                @Override
                public void run() {
                    waitingNotifications.add(post);
                }
            });
            ensurePermission("android.permission.POST_NOTIFICATIONS");
        } else {
            runOnUiThread(post);
        }
    }

    public void cancelNotification(int id) {
        android.app.NotificationManager manager = (android.app.NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
        manager.cancel(id);
    }

    /**
     * Mostra ou esconde o teclado (o aylis chama de qualquer thread). kind: 0 texto,
     * 1 senha, 2 número, 3 e-mail.
     */
    public void showKeyboard(final boolean show, final int kind) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                InputMethodManager imm = (InputMethodManager) getSystemService(Context.INPUT_METHOD_SERVICE);
                if (imm == null) {
                    return;
                }
                if (show) {
                    input.kind = kind;
                    input.requestFocus();
                    imm.restartInput(input);
                    imm.showSoftInput(input, 0);
                } else {
                    imm.hideSoftInputFromWindow(input.getWindowToken(), 0);
                    // Sem foco, o Android não reabre o teclado quando o app volta.
                    input.clearFocus();
                }
            }
        });
    }

    /**
     * As barras do sistema (o SystemBars do Aylis): a de status com statusColor e a de
     * navegação com navColor; light* = fundo claro (ícones escuros); show* = aparece (escondida,
     * volta deslizando da borda, por um tempo).
     */
    public void setSystemBars(final int statusColor, final int navColor, final boolean lightStatus, final boolean lightNav,
            final boolean showStatus, final boolean showNav) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                Window w = getWindow();
                w.addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);
                w.setStatusBarColor(statusColor);
                w.setNavigationBarColor(navColor);
                if (Build.VERSION.SDK_INT >= 30) {
                    WindowInsetsController c = w.getInsetsController();
                    if (c != null) {
                        int appearance = (lightStatus ? WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS : 0)
                                | (lightNav ? WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS : 0);
                        c.setSystemBarsAppearance(appearance,
                                WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS | WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS);
                        c.setSystemBarsBehavior(WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
                        if (showStatus) c.show(WindowInsets.Type.statusBars()); else c.hide(WindowInsets.Type.statusBars());
                        if (showNav) c.show(WindowInsets.Type.navigationBars()); else c.hide(WindowInsets.Type.navigationBars());
                    }
                } else {
                    View d = w.getDecorView();
                    int v = d.getSystemUiVisibility();
                    v = lightStatus ? (v | View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR) : (v & ~View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR);
                    v = lightNav ? (v | View.SYSTEM_UI_FLAG_LIGHT_NAVIGATION_BAR) : (v & ~View.SYSTEM_UI_FLAG_LIGHT_NAVIGATION_BAR);
                    int hide = (showStatus ? 0 : View.SYSTEM_UI_FLAG_FULLSCREEN) | (showNav ? 0 : View.SYSTEM_UI_FLAG_HIDE_NAVIGATION);
                    v = (v & ~(View.SYSTEM_UI_FLAG_FULLSCREEN | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION | View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY)) | hide
                            | (hide != 0 ? View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY : 0);
                    d.setSystemUiVisibility(v);
                }
            }
        });
    }

    static class InputView extends View {
        int kind;

        InputView(Context context) {
            super(context);
            setFocusable(true);
            setFocusableInTouchMode(true);
        }

        @Override
        public boolean onCheckIsTextEditor() {
            return true;
        }

        @Override
        public InputConnection onCreateInputConnection(EditorInfo info) {
            switch (kind) {
                case 1:
                    info.inputType = InputType.TYPE_CLASS_TEXT | InputType.TYPE_TEXT_VARIATION_PASSWORD;
                    break;
                case 2:
                    info.inputType = InputType.TYPE_CLASS_NUMBER | InputType.TYPE_NUMBER_FLAG_DECIMAL | InputType.TYPE_NUMBER_FLAG_SIGNED;
                    break;
                case 3:
                    info.inputType = InputType.TYPE_CLASS_TEXT | InputType.TYPE_TEXT_VARIATION_EMAIL_ADDRESS;
                    break;
                case 4:
                    // Sem sugestões: vários teclados ignoram só o NO_SUGGESTIONS; com a
                    // variação de senha visível todos desligam a faixa e a correção.
                    info.inputType = InputType.TYPE_CLASS_TEXT | InputType.TYPE_TEXT_FLAG_NO_SUGGESTIONS | InputType.TYPE_TEXT_VARIATION_VISIBLE_PASSWORD;
                    break;
                default:
                    info.inputType = InputType.TYPE_CLASS_TEXT | InputType.TYPE_TEXT_FLAG_AUTO_CORRECT | InputType.TYPE_TEXT_FLAG_CAP_SENTENCES;
            }
            info.imeOptions = EditorInfo.IME_FLAG_NO_FULLSCREEN | EditorInfo.IME_FLAG_NO_EXTRACT_UI | EditorInfo.IME_ACTION_DONE;
            info.initialSelStart = 0;
            info.initialSelEnd = 0;
            return new Connection(this);
        }
    }

    /**
     * Guarda uma cópia do que foi digitado (o teclado consulta para sugerir e corrigir) e
     * manda cada mudança ao aylis como "apaga antes, apaga depois, insere".
     */
    static class Connection extends BaseInputConnection {
        Connection(View view) {
            super(view, true);
        }

        /** O trecho que a próxima edição troca: o texto em composição, ou a seleção. */
        private void replace(CharSequence text) {
            Editable e = getEditable();
            int a = getComposingSpanStart(e);
            int b = getComposingSpanEnd(e);
            if (a < 0 || b < 0) {
                a = Selection.getSelectionStart(e);
                b = Selection.getSelectionEnd(e);
            }
            int start = Math.min(a, b);
            int end = Math.max(a, b);
            int cursor = Selection.getSelectionEnd(e);
            if (cursor < 0) {
                cursor = end;
            }
            nativeEdit(Math.max(0, cursor - start), Math.max(0, end - cursor), text.toString());
        }

        @Override
        public boolean setComposingText(CharSequence text, int newCursorPosition) {
            replace(text);
            return super.setComposingText(text, newCursorPosition);
        }

        @Override
        public boolean commitText(CharSequence text, int newCursorPosition) {
            replace(text);
            return super.commitText(text, newCursorPosition);
        }

        @Override
        public boolean deleteSurroundingText(int before, int after) {
            nativeEdit(before, after, "");
            return super.deleteSurroundingText(before, after);
        }

        @Override
        public boolean sendKeyEvent(KeyEvent event) {
            if (event.getAction() == KeyEvent.ACTION_DOWN) {
                int code = event.getKeyCode();
                int ch = event.getUnicodeChar();
                if (code == KeyEvent.KEYCODE_DEL || code == KeyEvent.KEYCODE_FORWARD_DEL || code == KeyEvent.KEYCODE_ENTER
                        || code == KeyEvent.KEYCODE_TAB || (code >= KeyEvent.KEYCODE_DPAD_UP && code <= KeyEvent.KEYCODE_DPAD_RIGHT)
                        || code == KeyEvent.KEYCODE_MOVE_HOME || code == KeyEvent.KEYCODE_MOVE_END || ch == 0) {
                    nativeKey(code);
                } else {
                    nativeEdit(0, 0, new String(Character.toChars(ch)));
                }
            }
            return true;
        }

        @Override
        public boolean performEditorAction(int action) {
            nativeKey(KeyEvent.KEYCODE_ENTER);
            return true;
        }
    }
}

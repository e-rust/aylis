// Cliente do protocolo do aylis para backends em C++17, via aylis-host. Um cabeçalho só,
// sem dependências (traz um JSON mínimo).
//
// O aylis-host inicia este processo e conversa por stdin/stdout: uma mensagem JSON por linha.
//
//     aylis::Aylis app;
//     app.send("load", {{"layout", aylis::Json::parse(texto)}});
//     auto name = app.call("get", {{"target", "edtName"}, {"prop", "text"}}).str();
//     app.on("btn_salvar_click", [&](const aylis::Event& ev) { ... });
//     app.run("frmClientes");
//
// Os handlers rodam na thread do run/loop.

#pragma once

#include <cerrno>
#include <cstdio>
#include <algorithm>
#include <array>
#include <cmath>
#include <charconv>
#include <condition_variable>
#include <deque>
#include <functional>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <thread>
#include <utility>
#include <vector>

#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif

namespace aylis {

/// Uma linha do stdin. A leitura interrompida por um sinal (`EINTR`: o depurador que se anexa
/// no macOS para o processo com SIGSTOP) é repetida, em vez de valer como a tela fechada.
inline bool read_line(std::string& line) {
    for (;;) {
        errno = 0;
        if (std::getline(std::cin, line)) return true;
        // O std::cin marca fim de arquivo nos dois casos; o FILE do stdin distingue.
        if (std::feof(stdin) || errno != EINTR) return false;
        std::cin.clear();
        std::clearerr(stdin);
    }
}

/// Um valor JSON.
class Json {
public:
    enum class Kind { Null, Bool, Number, String, Array, Object };
    using Object = std::vector<std::pair<std::string, Json>>;

    Json() = default;
    Json(std::nullptr_t) {}
    Json(bool b) : kind_(Kind::Bool), bool_(b) {}
    Json(int n) : kind_(Kind::Number), number_(n) {}
    Json(long long n) : kind_(Kind::Number), number_(static_cast<double>(n)) {}
    Json(double n) : kind_(Kind::Number), number_(n) {}
    Json(const char* s) : kind_(Kind::String), string_(s) {}
    Json(std::string s) : kind_(Kind::String), string_(std::move(s)) {}
    Json(std::initializer_list<std::pair<const std::string, Json>> o) : kind_(Kind::Object) {
        for (auto& [k, v] : o) object_.emplace_back(k, v);
    }
    static Json array(std::vector<Json> items) {
        Json j;
        j.kind_ = Kind::Array;
        j.array_ = std::move(items);
        return j;
    }
    static Json object(Object items = {}) {
        Json j;
        j.kind_ = Kind::Object;
        j.object_ = std::move(items);
        return j;
    }

    Kind kind() const { return kind_; }
    bool is_null() const { return kind_ == Kind::Null; }
    double number() const { return number_; }
    bool boolean() const { return bool_; }
    /// O texto (ou a representação JSON, se não for texto).
    std::string str() const { return kind_ == Kind::String ? string_ : (is_null() ? std::string() : dump()); }
    const std::vector<Json>& items() const { return array_; }

    const Json& operator[](const std::string& key) const {
        static const Json null;
        for (auto& [k, v] : object_)
            if (k == key) return v;
        return null;
    }
    void set(const std::string& key, Json value) {
        if (kind_ != Kind::Object) *this = object();
        for (auto& [k, v] : object_)
            if (k == key) {
                v = std::move(value);
                return;
            }
        object_.emplace_back(key, std::move(value));
    }
    const Object& members() const { return object_; }

    std::string dump() const {
        std::ostringstream out;
        write(out);
        return out.str();
    }

    static Json parse(const std::string& text) {
        size_t i = 0;
        Json v = parse_value(text, i);
        return v;
    }

private:
    Kind kind_ = Kind::Null;
    bool bool_ = false;
    double number_ = 0;
    std::string string_;
    std::vector<Json> array_;
    Object object_;

    static void write_string(std::ostream& out, const std::string& s) {
        out << '"';
        for (unsigned char c : s) {
            switch (c) {
                case '"': out << "\\\""; break;
                case '\\': out << "\\\\"; break;
                case '\n': out << "\\n"; break;
                case '\r': out << "\\r"; break;
                case '\t': out << "\\t"; break;
                default:
                    if (c < 0x20) {
                        char buf[8];
                        std::snprintf(buf, sizeof buf, "\\u%04x", c);
                        out << buf;
                    } else {
                        out << c;  // UTF-8 passa como está
                    }
            }
        }
        out << '"';
    }

    void write(std::ostream& out) const {
        switch (kind_) {
            case Kind::Null: out << "null"; break;
            case Kind::Bool: out << (bool_ ? "true" : "false"); break;
            case Kind::Number:
                if (number_ == static_cast<double>(static_cast<long long>(number_))) out << static_cast<long long>(number_);
                else {
                    // O menor texto que volta ao mesmo double (o << padrão corta em 6 dígitos).
                    char buf[32];
                    auto r = std::to_chars(buf, buf + sizeof buf, number_);
                    out.write(buf, r.ptr - buf);
                }
                break;
            case Kind::String: write_string(out, string_); break;
            case Kind::Array:
                out << '[';
                for (size_t k = 0; k < array_.size(); ++k) {
                    if (k) out << ',';
                    array_[k].write(out);
                }
                out << ']';
                break;
            case Kind::Object:
                out << '{';
                for (size_t k = 0; k < object_.size(); ++k) {
                    if (k) out << ',';
                    write_string(out, object_[k].first);
                    out << ':';
                    object_[k].second.write(out);
                }
                out << '}';
                break;
        }
    }

    static void space(const std::string& s, size_t& i) {
        while (i < s.size() && (s[i] == ' ' || s[i] == '\n' || s[i] == '\r' || s[i] == '\t')) ++i;
    }

    static void utf8(std::string& out, unsigned cp) {
        if (cp < 0x80) out += static_cast<char>(cp);
        else if (cp < 0x800) {
            out += static_cast<char>(0xC0 | (cp >> 6));
            out += static_cast<char>(0x80 | (cp & 0x3F));
        } else if (cp < 0x10000) {
            out += static_cast<char>(0xE0 | (cp >> 12));
            out += static_cast<char>(0x80 | ((cp >> 6) & 0x3F));
            out += static_cast<char>(0x80 | (cp & 0x3F));
        } else {
            out += static_cast<char>(0xF0 | (cp >> 18));
            out += static_cast<char>(0x80 | ((cp >> 12) & 0x3F));
            out += static_cast<char>(0x80 | ((cp >> 6) & 0x3F));
            out += static_cast<char>(0x80 | (cp & 0x3F));
        }
    }

    static std::string parse_string(const std::string& s, size_t& i) {
        std::string out;
        ++i;  // '"'
        while (i < s.size() && s[i] != '"') {
            char c = s[i++];
            if (c != '\\') {
                out += c;
                continue;
            }
            char e = s[i++];
            switch (e) {
                case 'n': out += '\n'; break;
                case 'r': out += '\r'; break;
                case 't': out += '\t'; break;
                case 'b': out += '\b'; break;
                case 'f': out += '\f'; break;
                case 'u': {
                    unsigned cp = std::stoul(s.substr(i, 4), nullptr, 16);
                    i += 4;
                    // Par substituto (emoji): 😀.
                    if (cp >= 0xD800 && cp < 0xDC00 && s.compare(i, 2, "\\u") == 0) {
                        unsigned lo = std::stoul(s.substr(i + 2, 4), nullptr, 16);
                        cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                        i += 6;
                    }
                    utf8(out, cp);
                    break;
                }
                default: out += e;
            }
        }
        ++i;  // '"'
        return out;
    }

    static Json parse_value(const std::string& s, size_t& i) {
        space(s, i);
        if (i >= s.size()) return Json();
        char c = s[i];
        if (c == '{') {
            ++i;
            Json o = object();
            space(s, i);
            if (s[i] == '}') {
                ++i;
                return o;
            }
            for (;;) {
                space(s, i);
                std::string key = parse_string(s, i);
                space(s, i);
                ++i;  // ':'
                o.object_.emplace_back(key, parse_value(s, i));
                space(s, i);
                if (s[i++] == '}') return o;
            }
        }
        if (c == '[') {
            ++i;
            std::vector<Json> items;
            space(s, i);
            if (s[i] == ']') {
                ++i;
                return array(items);
            }
            for (;;) {
                items.push_back(parse_value(s, i));
                space(s, i);
                if (s[i++] == ']') return array(items);
            }
        }
        if (c == '"') return Json(parse_string(s, i));
        if (s.compare(i, 4, "true") == 0) {
            i += 4;
            return Json(true);
        }
        if (s.compare(i, 5, "false") == 0) {
            i += 5;
            return Json(false);
        }
        if (s.compare(i, 4, "null") == 0) {
            i += 4;
            return Json();
        }
        size_t start = i;
        while (i < s.size() && std::string("+-0123456789.eE").find(s[i]) != std::string::npos) ++i;
        return Json(std::stod(s.substr(start, i - start)));
    }
};

/// O ponteiro de um `on_click` (o campo `pointer` do evento): `x`/`y` no componente,
/// `form_x`/`form_y` no form, `button` (`primary`, `secondary`, `middle`...), as teclas
/// apertadas (`ctrl`, `shift`, `alt`, `meta`) e `kind` (`mouse`, `touch`, `pen`).
struct Pointer {
    double x = 0, y = 0, form_x = 0, form_y = 0;
    std::string button, kind;
    std::vector<std::string> modifiers;
    int clicks = 0;

    bool has(const std::string& modifier) const {
        return std::find(modifiers.begin(), modifiers.end(), modifier) != modifiers.end();
    }
    bool ctrl() const { return has("ctrl"); }
    bool shift() const { return has("shift"); }
    bool alt() const { return has("alt"); }
    bool meta() const { return has("meta"); }

    static Pointer from(const Json& p) {
        Pointer out;
        out.x = p["x"].number();
        out.y = p["y"].number();
        out.form_x = p["form_x"].number();
        out.form_y = p["form_y"].number();
        out.button = p["button"].str();
        out.kind = p["kind"].str();
        for (const Json& m : p["modifiers"].items()) out.modifiers.push_back(m.str());
        out.clicks = static_cast<int>(p["clicks"].number());
        return out;
    }
};

/// Um evento da tela.
struct Event {
    std::string event, handler, form, sender_name, sender_class;
    Json args;
    /// Dados extras de alguns eventos (a URL de quem mandou, no on_message do WebView).
    Json info;
    /// Num evento que pergunta (`on_close_query`, `before_post`, `on_key_down`...), o número da
    /// pergunta (-1 nos outros): a tela espera a resposta, que o cliente manda quando o handler
    /// termina.
    long long ask = -1;
    /// A resposta, gravável pelo handler: `ev.reply.set("can_close", false)` (fechar),
    /// `ev.reply.set("key", "")` (engolir a tecla) ou outro caractere (trocar), `data`/`label` (o
    /// arrasto), `accept` (soltar aqui). Recusar com mensagem (`before_post`):
    /// `throw std::runtime_error("Informe o CPF")`.
    mutable Json reply = Json::object();
    /// No `on_click` do mouse, do toque ou da caneta, onde, com que botão e com que teclas
    /// (`if (ev.has_pointer && ev.pointer.ctrl()) ...`); sem ele (`has_pointer` falso) no
    /// clique do teclado ou do código.
    bool has_pointer = false;
    Pointer pointer;
    /// O id de quem disparou (o do componente no form).
    long long sender_id = 0;
};

class Error : public std::runtime_error {
public:
    using std::runtime_error::runtime_error;
};

/// Escreve no console do aylis-host (o stdout é do protocolo).
inline void log(const std::string& text) {
    std::cerr << "[backend] " << text << std::endl;
}

class Aylis;

/// Um componente de um form aberto: o id dele na tela (as classes geradas no components.hpp
/// têm as propriedades). Um vazio (sem `app_`) é o que o find_x devolve para um nome que não há.
/// Uma cor do acesso tipado: RGBA (0 a 255) ou um nome do tema (`Color::of_token("primary")`),
/// que acompanha a troca de tema.
struct Color {
    unsigned char r = 0, g = 0, b = 0, a = 255;
    std::string token;

    static Color rgb(unsigned char r, unsigned char g, unsigned char b, unsigned char a = 255) { return Color{r, g, b, a, {}}; }
    static Color of_token(const std::string& name) { return Color{0, 0, 0, 255, name}; }
    /// `#RRGGBB`, `#RRGGBBAA` ou `#RGB` (inválida: preta).
    static Color hex(std::string h) {
        if (!h.empty() && h[0] == '#') h.erase(0, 1);
        if (h.size() == 3) h = std::string{h[0], h[0], h[1], h[1], h[2], h[2]};
        auto n = [&](size_t i) { return static_cast<unsigned char>(std::stoi(h.substr(i, 2), nullptr, 16)); };
        if (h.size() != 6 && h.size() != 8) return Color{};
        try {
            return Color{n(0), n(2), n(4), h.size() == 8 ? n(6) : static_cast<unsigned char>(255), {}};
        } catch (...) {
            return Color{};
        }
    }
    /// O valor da tela (`#RRGGBB[AA]` ou um nome).
    static Color from_json(const Json& v) {
        std::string s = v.kind() == Json::Kind::String ? v.str() : std::string();
        if (s.empty()) return Color{};
        return s[0] == '#' ? hex(s) : of_token(s);
    }
    std::string to_json() const {
        if (!token.empty()) return token;
        char out[10];
        if (a == 255)
            std::snprintf(out, sizeof out, "#%02X%02X%02X", r, g, b);
        else
            std::snprintf(out, sizeof out, "#%02X%02X%02X%02X", r, g, b, a);
        return out;
    }
    bool operator==(const Color& o) const { return to_json() == o.to_json(); }
};

/// Margens (`margins`, `padding`): esquerda, cima, direita e baixo.
struct Bounds {
    double left = 0, top = 0, right = 0, bottom = 0;

    static Bounds all(double v) { return Bounds{v, v, v, v}; }
    static Bounds symmetric(double h, double v) { return Bounds{h, v, h, v}; }
    /// O valor da tela: um número, `[h, v]`, `[l, t, r, b]` ou um objeto.
    static Bounds from_json(const Json& v) {
        if (v.kind() == Json::Kind::Number) return all(v.number());
        if (v.kind() == Json::Kind::Array && v.items().size() == 2) return symmetric(v.items()[0].number(), v.items()[1].number());
        if (v.kind() == Json::Kind::Array && v.items().size() == 4) return Bounds{v.items()[0].number(), v.items()[1].number(), v.items()[2].number(), v.items()[3].number()};
        if (v.kind() == Json::Kind::Object) return Bounds{v["left"].number(), v["top"].number(), v["right"].number(), v["bottom"].number()};
        return Bounds{};
    }
    Json to_json() const { return Json::array({Json(left), Json(top), Json(right), Json(bottom)}); }
    bool operator==(const Bounds& o) const { return left == o.left && top == o.top && right == o.right && bottom == o.bottom; }
};

inline std::string num_(double v) {
    char out[32];
    std::snprintf(out, sizeof out, "%g", std::round(v * 1000) / 1000);
    return out;
}

/// Uma sombra: o deslocamento (`dx`, `dy`), o desfoque, o espalhamento e a cor (sem cor, preto a
/// 25%). Vazia (`std::nullopt`) no acesso tipado é sem sombra.
struct Shadow {
    double dx = 0, dy = 0, blur = 0, spread = 0;
    Color color = Color::rgb(0, 0, 0, 64);

    /// `"dx dy desfoque [espalhamento] [#cor]"` (vazio ou inválido: nullopt).
    static std::optional<Shadow> from_json(const Json& v) {
        std::string text = v.kind() == Json::Kind::String ? v.str() : std::string();
        for (auto& c : text)
            if (c == ',') c = ' ';
        std::istringstream in(text);
        std::vector<double> numbers;
        Shadow s;
        bool has_color = false;
        std::string token;
        while (in >> token) {
            if (token.size() > 2 && token.compare(token.size() - 2, 2, "px") == 0) token.resize(token.size() - 2);
            char* end = nullptr;
            double d = std::strtod(token.c_str(), &end);
            if (end && *end == '\0' && end != token.c_str())
                numbers.push_back(d);
            else if (!has_color && token[0] == '#') {
                s.color = Color::hex(token);
                has_color = true;
            } else
                return std::nullopt;
        }
        if (numbers.size() < 2 || numbers.size() > 4) return std::nullopt;
        s.dx = numbers[0];
        s.dy = numbers[1];
        if (numbers.size() > 2) s.blur = numbers[2];
        if (numbers.size() > 3) s.spread = numbers[3];
        return s;
    }
    std::string to_json() const {
        std::string out = num_(dx) + " " + num_(dy) + " " + num_(blur);
        if (spread != 0) out += " " + num_(spread);
        return out + " " + color.to_json();
    }
};

/// Uma curva de suavização: uma das prontas (`Easing::named("out_quad")`) ou a Bézier cúbica
/// (`Easing::cubic_bezier(0.2, 0, 0, 1)`), como o `cubic-bezier()` do CSS.
struct Easing {
    std::string name = "linear";
    std::optional<std::array<double, 4>> bezier;

    static Easing named(const std::string& n) { return Easing{n, std::nullopt}; }
    static Easing cubic_bezier(double x1, double y1, double x2, double y2) { return Easing{"", std::array<double, 4>{x1, y1, x2, y2}}; }
    static Easing from_json(const Json& v) {
        std::string text = v.kind() == Json::Kind::String ? v.str() : std::string();
        double b[4];
        if (text.rfind("cubic-bezier(", 0) == 0 && std::sscanf(text.c_str() + 13, "%lf , %lf , %lf , %lf", &b[0], &b[1], &b[2], &b[3]) == 4)
            return cubic_bezier(b[0], b[1], b[2], b[3]);
        if (text == "ease") return cubic_bezier(0.25, 0.1, 0.25, 1);
        if (text == "ease-in") return cubic_bezier(0.42, 0, 1, 1);
        if (text == "ease-out") return cubic_bezier(0, 0, 0.58, 1);
        if (text == "ease-in-out") return cubic_bezier(0.42, 0, 0.58, 1);
        for (auto& c : text)
            if (c == '-') c = '_';
        for (const char* n : {"linear", "in_quad", "out_quad", "in_out_quad", "out_cubic", "in_out_cubic", "out_back"})
            if (text == n) return named(n);
        return named("linear");
    }
    std::string to_json() const {
        if (!bezier) return name;
        auto& b = *bezier;
        return "cubic-bezier(" + num_(b[0]) + ", " + num_(b[1]) + ", " + num_(b[2]) + ", " + num_(b[3]) + ")";
    }
};

/// Um ponto (`x`, `y`).
struct Point {
    double x = 0, y = 0;

    /// `"0.5,0 1,1 0,1"` (os que não são pontos ficam de fora).
    static std::vector<Point> list_from_json(const Json& v) {
        std::vector<Point> out;
        std::istringstream in(v.kind() == Json::Kind::String ? v.str() : std::string());
        std::string pair;
        while (in >> pair) {
            Point p;
            if (std::sscanf(pair.c_str(), "%lf,%lf", &p.x, &p.y) == 2) out.push_back(p);
        }
        return out;
    }
    static std::string list_to_json(const std::vector<Point>& points) {
        std::string out;
        for (size_t i = 0; i < points.size(); ++i) out += (i ? " " : "") + num_(points[i].x) + "," + num_(points[i].y);
        return out;
    }
};

struct Component {
    Aylis* app_ = nullptr;
    long long id_ = 0;
    std::string name_;

    Component() = default;
    Component(Aylis* app, long long id, std::string name) : app_(app), id_(id), name_(std::move(name)) {}
    explicit operator bool() const { return app_ != nullptr; }
    /// O id do componente na tela (o alvo dos comandos).
    long long id() const { return id_; }
    /// O nome do componente no form.
    const std::string& name() const { return name_; }

    Json get_prop_(const std::string& prop) const;
    void set_prop_(const std::string& prop, Json value) const;
    Json ask_(const std::string& cmd, Json fields) const;
    void tell_(const std::string& cmd, Json fields) const;
    /// A referência a outro componente (`popup_menu`, `data_source`); vazia se não há.
    std::optional<Component> get_ref_(const std::string& prop) const;
    void set_ref_(const std::string& prop, const std::optional<Component>& component) const;
    /// O componente que a tela devolveu com `"ref": true` (o id e o nome); vazio se não há.
    std::optional<Component> component_of_(const Json& info) const {
        if (info.kind() != Json::Kind::Object) return std::nullopt;
        return Component(app_, static_cast<long long>(info["id"].number()), info["name"].str());
    }
};

/// A base dos forms: cada form aberto na tela tem a sua instância
/// (duas janelas do mesmo form, duas instâncias), com os componentes resolvidos ao abrir. A
/// classe gerada de cada form monta os componentes em `bind`.
struct FormBase : Component {
    using Component::Component;
    FormBase() = default;
    virtual ~FormBase() = default;

    /// Monta os componentes a partir da lista da tela (a classe gerada sobrescreve).
    virtual void bind(const Json& list) { (void)list; }

protected:
    /// O componente do form; o que falta ou mudou de classe: o form não bate com o código.
    Component resolve_(const Json& list, const std::string& name, const std::string& cls) const {
        const Json* item = find_(list, name);
        if (!item) throw std::runtime_error("o form " + name_ + " não bate com o código: falta o componente " + name + " (" + cls + ")");
        if ((*item)["class"].str() != cls) throw std::runtime_error("o form " + name_ + " não bate com o código: " + name + " é " + (*item)["class"].str() + " no form e " + cls + " no código");
        return Component(app_, static_cast<long long>((*item)["id"].number()), name);
    }

    /// Um componente que só existe em algumas plataformas (vazio se não está aqui).
    Component resolve_optional_(const Json& list, const std::string& name) const {
        const Json* item = find_(list, name);
        return item ? Component(app_, static_cast<long long>((*item)["id"].number()), name) : Component();
    }

private:
    static const Json* find_(const Json& list, const std::string& name) {
        for (const Json& c : list.items())
            if (c["name"].str() == name) return &c;
        return nullptr;
    }
};

/// A conexão com a tela.
class Aylis {
public:
    /// Como criar a instância de um form e como torná-la a atual (o frmX gerado passa a ser a
    /// janela do evento).
    using FormMaker = std::function<std::unique_ptr<FormBase>(Aylis*, long long, const std::string&)>;
    using FormActivator = std::function<void(FormBase&)>;

    /// Registra a classe de um form (o components.hpp gerado registra): cada form com esse
    /// nome aberto na tela vira uma instância dela.
    static void register_form_class(const std::string& name, FormMaker make, FormActivator activate) {
        form_classes()[name] = {std::move(make), std::move(activate)};
    }

    /// A instância da janela `id` do form (criada e ligada na primeira vez) e torna-a a atual.
    FormBase* form_instance(const std::string& form, long long id) {
        auto entry = form_classes().find(form);
        if (entry == form_classes().end()) return nullptr;
        auto it = forms_.find(id);
        if (it == forms_.end()) {
            auto instance = entry->second.first(this, id, form);
            instance->bind(call("components", {{"target", Json(id)}}));
            it = forms_.emplace(id, std::move(instance)).first;
        }
        entry->second.second(*it->second);
        return it->second.get();
    }

    /// Cria o form na tela e devolve a instância dele; `another`: mais uma janela do mesmo form.
    FormBase* create_form(const std::string& form, bool another = false) {
        Json created = call("create_form", {{"name", form}, {"new", another}});
        return form_instance(form, static_cast<long long>(created["id"].number()));
    }

    Aylis() {
#ifdef _WIN32
        // Bytes como estão (UTF-8), sem a tradução de fim de linha.
        _setmode(_fileno(stdin), _O_BINARY);
        _setmode(_fileno(stdout), _O_BINARY);
#endif
    }

    /// Envia um comando sem esperar resposta (erros chegam como evento "error").
    void send(const std::string& cmd, Json fields = Json::object()) { write(message(cmd, std::move(fields))); }

    /// Um trecho marcado no profiler: termina quando sai do escopo.
    class Mark {
    public:
        explicit Mark(Aylis* app) : app_(app) {}
        Mark(Mark&& other) noexcept : app_(other.app_) { other.app_ = nullptr; }
        Mark(const Mark&) = delete;
        Mark& operator=(const Mark&) = delete;
        ~Mark() {
            if (app_) app_->send("profile_end");
        }

    private:
        Aylis* app_;
    };

    /// Marca um trecho no profiler da IDE (a faixa "Marcas"): `auto p = app.profile("consulta");`.
    Mark profile(const std::string& name) {
        Json fields = Json::object();
        fields.set("name", name);
        send("profile_begin", std::move(fields));
        return Mark(this);
    }

    /// Envia um comando e espera o resultado (só na thread do loop).
    Json call(const std::string& cmd, Json fields = Json::object()) {
        // A resposta chega pela leitura da thread do loop: de outra thread, as duas leriam a
        // mesma entrada (use post).
        if (owner_ == std::thread::id()) owner_ = std::this_thread::get_id();
        else if (owner_ != std::this_thread::get_id()) throw Error("call(\"" + cmd + "\") de outra thread: leia na thread do loop (post) ou antes de iniciar a thread");
        long long id = ++next_id_;
        Json m = message(cmd, std::move(fields));
        m.set("id", Json(id));
        write(m);
        for (;;) {
            Json reply;
            if (!read(reply)) throw Error("a tela foi fechada");
            const Json& got = reply["id"];
            if (got.kind() == Json::Kind::Number && static_cast<long long>(got.number()) == id) {
                if (!reply["error"].is_null()) throw Error(reply["error"].str());
                return reply["result"];
            }
            // Esperando um diálogo, os eventos (os do modal) rodam já; um evento que pergunta
            // também (o post do próprio backend dispara o before_post).
            const std::string kind = reply["event"].str();
            if (kind == "__post" && waiting_ > 0) {
                run_posted();
                continue;
            }
            if (kind == "ping" && waiting_ > 0) {
                send("pong");  // a sincronia dos roteiros: o que veio antes já rodou
                continue;
            }
            if (!kind.empty() && kind != "closed" && kind != "ping" && kind != "__post" && (waiting_ > 0 || (!reply["ask"].is_null() && running_ < 8))) {
                handle_event(reply);
                continue;
            }
            queued_.push_back(reply);
        }
    }

    /// O valor de um campo decimal (Currency, BCD) como o texto exato (`"1234.50"`): o C++ não tem
    /// decimal, e o `double` perderia centavos; use uma biblioteca de decimal com ele. Vazio se o
    /// campo é nulo.
    std::string field_decimal(const std::string& data_set, const std::string& field) {
        return call("data", {{"target", data_set}, {"op", "value"}, {"field", field}}).str();
    }

    /// Caixa de mensagem que espera: devolve o botão (`"yes"`,
    /// `"no"`, `"ok"`...). Enquanto espera, os eventos continuam rodando.
    std::string message_dlg(const std::string& text, const std::string& kind = "information", std::vector<std::string> buttons = {"ok"}) {
        std::vector<Json> list(buttons.begin(), buttons.end());
        return wait_for("message_dlg", {{"text", text}, {"kind", kind}, {"buttons", Json::array(list)}});
    }

    /// Abre o form como modal e espera ele fechar: devolve o
    /// resultado (`"ok"`, `"cancel"`...).
    std::string show_modal(const Component& form) { return wait_for("show_modal", {{"target", Json(form.id_)}}); }

    /// Handler pelo nome usado no layout ("frmClientes.nome": só daquele form).
    void on(const std::string& name, std::function<void(const Event&)> handler) { handlers_[name] = std::move(handler); }

    /// Abre o form e trata os eventos até a tela fechar ("": o principal do projeto).
    void run(const std::string& form = "") {
        call("run", {{"form", form.empty() ? Json() : Json(form)}});
        // A instância do form principal antes do primeiro evento (o código pode usar frmX).
        if (!form.empty()) {
            Json found = call("find", {{"name", form}});
            if (found.kind() == Json::Kind::Number) form_instance(form, static_cast<long long>(found.number()));
        }
        loop();
    }

    /// Trata os eventos até a tela fechar.
    void loop() {
        for (;;) {
            Json m;
            if (!queued_.empty()) {
                m = queued_.front();
                queued_.pop_front();
            } else if (!read(m)) {
                return;
            }
            const std::string kind = m["event"].str();
            if (kind.empty()) continue;
            if (kind == "closed") return;
            if (kind == "__post") {
                run_posted();
                continue;
            }
            // Sincronia (roteiros de teste): responde depois de tratar tudo que veio antes.
            if (kind == "ping") {
                send("pong");
                continue;
            }
            if (kind == "error") {
                log("erro na tela: " + m["message"].str());
                continue;
            }
            handle_event(m);
        }
    }

    /// Roda os `post` pendentes.
    void run_posted() {
        Inbox* box = inbox_ ? inbox_.get() : desktop_.get();
        for (;;) {
            std::function<void()> job;
            {
                std::lock_guard<std::mutex> lock(box->mutex);
                if (box->posted.empty()) break;
                job = std::move(box->posted.front());
                box->posted.pop_front();
            }
            try {
                job();
            } catch (const std::exception& e) {
                log(std::string("erro no post: ") + e.what());
            }
        }
    }

    /// Roda o handler do evento. Um evento que pergunta (com `ask`) recebe a resposta quando o
    /// handler termina: o `ev.reply` ou, com exceção, a recusa com a mensagem (a tela mostra).
    void handle_event(const Json& m) {
        // Forms liberados na tela: as instâncias saem e os `post_to` deles não rodam.
        if (m["event"].str() == "__freed") {
            for (const Json& id : m["forms"].items()) {
                const long long n = static_cast<long long>(id.number());
                freed_.insert(n);
                forms_.erase(n);
            }
            return;
        }
        Event ev{m["event"].str(), m["handler"].str(), m["form"].str(), m["sender"]["name"].str(), m["sender"]["class"].str(), m["args"], m["info"]};
        if (m["ask"].kind() == Json::Kind::Number) ev.ask = static_cast<long long>(m["ask"].number());
        if (m["sender"]["id"].kind() == Json::Kind::Number) ev.sender_id = static_cast<long long>(m["sender"]["id"].number());
        if (m["pointer"].kind() == Json::Kind::Object) {
            ev.has_pointer = true;
            ev.pointer = Pointer::from(m["pointer"]);
        }
        auto it = handlers_.find(ev.form + "." + ev.handler);
        if (it == handlers_.end()) it = handlers_.find(ev.handler);
        ++running_;
        try {
            if (it == handlers_.end()) {
                log("evento sem handler: " + ev.handler);
            } else {
                const Json& form_id = m["form_id"];
                if (form_id.kind() == Json::Kind::Number) form_instance(ev.form, static_cast<long long>(form_id.number()));
                it->second(ev);
            }
            if (ev.ask >= 0) send("answer", {{"ask", Json(ev.ask)}, {"values", ev.reply}});
        } catch (const std::exception& e) {
            if (ev.ask >= 0) {
                send("answer", {{"ask", Json(ev.ask)}, {"error", std::string(e.what())}});
            } else {
                log("erro no handler " + ev.handler + ": " + e.what());
                send("exception", {{"message", std::string(e.what())}, {"path", ev.form}, {"handler", ev.handler}});
            }
        }
        --running_;
        // O handler terminou: a tela deixa de mostrar o backend ocupado.
        if (m["seq"].kind() == Json::Kind::Number) send("done", {{"seq", Json(static_cast<long long>(m["seq"].number()))}});
    }

    /// Como o `post`, ligado a um form: se ele foi liberado até lá (a janela fechou e saiu),
    /// `job` não roda.
    void post_to(const FormBase& owner, std::function<void()> job) {
        const long long id = owner.id_;
        post([this, id, job = std::move(job)] {
            if (!freed_.count(id)) job();
        });
    }

    /// Roda `job` na thread do loop desta tela (numa sessão; de qualquer thread).
    void post(std::function<void()> job) {
        // No desktop, a tela devolve um eco (wake) que acorda o loop.
        if (!inbox_) {
            {
                std::lock_guard<std::mutex> lock(desktop_->mutex);
                desktop_->posted.push_back(std::move(job));
            }
            send("wake");
            return;
        }
        {
            std::lock_guard<std::mutex> lock(inbox_->mutex);
            inbox_->posted.push_back(std::move(job));
            inbox_->messages.push_back(Json::object({{"event", Json("__post")}}));
        }
        inbox_->wake.notify_one();
    }

protected:
    /// Numa sessão: as mensagens chegam por aqui (o servidor entrega) e saem por `sink_`.
    struct Inbox {
        std::mutex mutex;
        std::condition_variable wake;
        std::deque<Json> messages;
        std::deque<std::function<void()>> posted;
    };
    std::shared_ptr<Inbox> inbox_;
    std::function<void(Json)> sink_;

    /// Entrega uma mensagem a esta tela (o servidor chama).
    void deliver(Json message) {
        {
            std::lock_guard<std::mutex> lock(inbox_->mutex);
            inbox_->messages.push_back(std::move(message));
        }
        inbox_->wake.notify_one();
    }

private:
    static std::map<std::string, std::pair<FormMaker, FormActivator>>& form_classes() {
        static std::map<std::string, std::pair<FormMaker, FormActivator>> classes;
        return classes;
    }
    /// As instâncias dos forms, pelo id do form na tela.
    std::map<long long, std::unique_ptr<FormBase>> forms_;
    /// Os forms liberados na tela (o aviso `__freed`).
    std::set<long long> freed_;
    std::map<std::string, std::function<void(const Event&)>> handlers_;
    /// Esperando um diálogo (os eventos rodam já) e os handlers rodando (um evento que pergunta
    /// roda mesmo durante um call).
    int waiting_ = 0;
    int running_ = 0;
    /// A thread que lê (a do loop).
    std::thread::id owner_;
    /// Os `post` do desktop (sem sessão).
    std::shared_ptr<Inbox> desktop_ = std::make_shared<Inbox>();

    std::string wait_for(const std::string& cmd, Json fields) {
        fields.set("wait", true);
        ++waiting_;
        // O que chegou antes (enquanto outro call esperava) roda já.
        for (size_t n = queued_.size(); n > 0; --n) {
            Json m = queued_.front();
            queued_.pop_front();
            const std::string kind = m["event"].str();
            if (kind == "__post") run_posted();
            else if (kind == "ping") send("pong");
            else if (!kind.empty() && kind != "closed" && kind != "error") handle_event(m);
            else queued_.push_back(m);
        }
        try {
            Json result = call(cmd, std::move(fields));
            --waiting_;
            return result.str();
        } catch (...) {
            --waiting_;
            throw;
        }
    }
    std::deque<Json> queued_;
    std::mutex write_lock_;
    /// A versão do protocolo do aylis que este cliente fala (o hello) e se já foi mandado.
    static constexpr int kProtocol = 1;
    bool greeted_ = false;
    long long next_id_ = 0;

    static Json message(const std::string& cmd, Json fields) {
        Json m = Json::object({{"cmd", Json(cmd)}});
        for (auto& [k, v] : fields.members()) m.set(k, v);
        return m;
    }

    void write(const Json& m) {
        // A primeira mensagem para a tela: a versão do protocolo (a tela recusa tudo antes).
        if (!greeted_) {
            greeted_ = true;
            write(Json::object({{"cmd", Json("hello")}, {"protocol", Json(kProtocol)}, {"client", Json("cpp")}, {"features", Json::array({Json("done")})}}));
        }
        if (sink_) {
            sink_(m);
            return;
        }
        std::lock_guard<std::mutex> lock(write_lock_);
        std::string text = m.dump();
        text += '\n';
        std::fwrite(text.data(), 1, text.size(), stdout);
        std::fflush(stdout);
    }

    bool read(Json& out) {
        if (inbox_) {
            std::unique_lock<std::mutex> lock(inbox_->mutex);
            inbox_->wake.wait(lock, [&] { return !inbox_->messages.empty(); });
            out = std::move(inbox_->messages.front());
            inbox_->messages.pop_front();
            return true;
        }
        std::string line;
        while (read_line(line)) {
            if (!line.empty() && line.back() == '\r') line.pop_back();
            if (line.empty()) continue;
            out = Json::parse(line);
            return true;
        }
        return false;
    }

    friend class Server;
};

class Server;
class User;

/// Uma tela da ponte com `--sessions` (`aylis-host --bridge ... --sessions`): um processo atende
/// todas as telas e cada uma roda numa thread própria (um handler demorado não trava as outras;
/// uma exceção fica nela). Guarde o estado da tela em variáveis do `started` passado ao
/// `Server::serve` (nunca em globais, que são de todas). A API é a do `Aylis`.
class Session : public Aylis {
public:
    Session(Server* server, int number);
    /// O número da tela na ponte.
    int number() const { return number_; }
    /// O usuário logado (o mesmo nas telas dele); nulo antes do `login`.
    std::shared_ptr<User> user() const { return user_; }
    /// Liga esta tela ao usuário `id`: as telas do mesmo usuário dividem o mesmo `User`.
    void login(const std::string& id);
    /// Solta o usuário desta tela (sozinho, ao fechar).
    void logout();

private:
    friend class Server;
    Server* server_;
    int number_;
    std::shared_ptr<User> user_;
};

/// O usuário logado, dividido pelas telas dele. `data` guarda o que vale para todas (o
/// carrinho, as preferências); use `std::lock_guard g(user->mutex)` quando duas telas podem
/// mexer ao mesmo tempo.
class User {
public:
    explicit User(std::string id) : id(std::move(id)) {}
    const std::string id;
    std::map<std::string, Json> data;
    std::mutex mutex;

    /// Roda `job(tela)` na thread de cada tela do usuário (atualizar todas).
    void broadcast(const std::function<void(Session&)>& job) {
        std::vector<Session*> list;
        {
            std::lock_guard<std::mutex> lock(mutex);
            list = sessions;
        }
        for (Session* s : list) s->post([s, job] { job(*s); });
    }

private:
    friend class Session;
    friend class Server;
    std::vector<Session*> sessions;
};

/// O processo das sessões: lê o stdin e entrega cada mensagem à tela dela.
class Server {
public:
    /// Atende as telas da ponte com `--sessions`: `started` roda numa thread para cada tela que
    /// conecta (monte o estado e chame `run`); `ended` (pode ser vazio) quando ela sai.
    static void serve(std::function<void(Session&)> started, std::function<void(Session&)> ended = {}) {
#ifdef _WIN32
        _setmode(_fileno(stdin), _O_BINARY);
        _setmode(_fileno(stdout), _O_BINARY);
#endif
        Server server;
        std::string line;
        while (read_line(line)) {
            if (!line.empty() && line.back() == '\r') line.pop_back();
            if (line.empty()) continue;
            Json parsed = Json::parse(line);
            if (parsed["session"].kind() != Json::Kind::Number) continue;
            int number = static_cast<int>(parsed["session"].number());
            Json message = Json::object();
            for (auto& [k, v] : parsed.members())
                if (k != "session") message.set(k, v);
            const std::string kind = message["event"].str();
            if (kind == "session_open") {
                auto session = std::make_shared<Session>(&server, number);
                {
                    std::lock_guard<std::mutex> lock(server.mutex_);
                    server.sessions_[number] = session;
                }
                std::thread([&server, session, number, started, ended] {
                    try {
                        started(*session);
                    } catch (const std::exception& e) {
                        log("erro na sessão " + std::to_string(number) + ": " + e.what());
                    }
                    try {
                        if (ended) ended(*session);
                    } catch (const std::exception& e) {
                        log("erro em ended da sessão " + std::to_string(number) + ": " + e.what());
                    }
                    session->logout();
                    std::lock_guard<std::mutex> lock(server.mutex_);
                    server.sessions_.erase(number);
                }).detach();
                continue;
            }
            std::shared_ptr<Session> target;
            {
                std::lock_guard<std::mutex> lock(server.mutex_);
                auto it = server.sessions_.find(number);
                if (it != server.sessions_.end()) target = it->second;
            }
            if (target) target->deliver(kind == "session_close" ? Json::object({{"event", Json("closed")}}) : message);
        }
    }

    void write(const Json& m) {
        std::lock_guard<std::mutex> lock(write_mutex_);
        std::string text = m.dump();
        text += '\n';
        std::fwrite(text.data(), 1, text.size(), stdout);
        std::fflush(stdout);
    }

    std::shared_ptr<User> user_of(const std::string& id) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto& u = users_[id];
        if (!u) u = std::make_shared<User>(id);
        return u;
    }

    void leave(const std::shared_ptr<User>& user, Session* session) {
        bool empty;
        {
            std::lock_guard<std::mutex> lock(user->mutex);
            auto& list = user->sessions;
            list.erase(std::remove(list.begin(), list.end(), session), list.end());
            empty = list.empty();
        }
        if (empty) {
            std::lock_guard<std::mutex> lock(mutex_);
            users_.erase(user->id);
        }
    }

private:
    std::mutex mutex_;
    std::mutex write_mutex_;
    std::map<int, std::shared_ptr<Session>> sessions_;
    std::map<std::string, std::shared_ptr<User>> users_;
};

inline Json Component::get_prop_(const std::string& prop) const { return app_->call("get", {{"target", Json(id_)}, {"prop", prop}}); }
inline void Component::set_prop_(const std::string& prop, Json value) const { app_->send("set", {{"target", Json(id_)}, {"props", {{prop, std::move(value)}}}}); }
inline Json Component::ask_(const std::string& cmd, Json fields) const { return app_->call(cmd, std::move(fields)); }
inline void Component::tell_(const std::string& cmd, Json fields) const { app_->send(cmd, std::move(fields)); }
inline std::optional<Component> Component::get_ref_(const std::string& prop) const {
    return component_of_(app_->call("get", {{"target", Json(id_)}, {"prop", prop}, {"ref", true}}));
}
inline void Component::set_ref_(const std::string& prop, const std::optional<Component>& component) const {
    set_prop_(prop, component ? Json(component->id()) : Json());
}

inline Session::Session(Server* server, int number) : server_(server), number_(number) {
    inbox_ = std::make_shared<Inbox>();
    sink_ = [this](Json m) {
        m.set("session", Json(number_));
        server_->write(m);
    };
}

inline void Session::login(const std::string& id) {
    logout();
    user_ = server_->user_of(id);
    std::lock_guard<std::mutex> lock(user_->mutex);
    user_->sessions.push_back(this);
}

inline void Session::logout() {
    if (user_) server_->leave(user_, this);
    user_.reset();
}

/// As alterações de um `apply_updates`, para o backend gravar do jeito dele, na ordem de gravar
/// (o mestre incluído antes dos itens; as exclusões do mestre por último). Cada alteração é um
/// objeto: `ref`, `op` ("insert", "update", "delete"), `data_set`, `key`, `old`, `new` e, no
/// item incluído sob um mestre também incluído, `master`. Ao aceitar o mestre com a chave
/// gerada, os itens dele já recebem a chave antes de serem gravados:
///
///     aylis::Delta delta(ev.args);
///     for (size_t i = 0; i < delta.size(); i++) {
///         const auto& c = delta.change(i);
///         if (c["op"].str() == "insert" && c["data_set"].str() == "cdsPedidos")
///             delta.accept(c, {{"id", inserir_pedido(c)}});
///     }
///     ev.reply.set("results", delta.results_json());
class Delta {
public:
    explicit Delta(Json source) {
        if (source.kind() == Json::Kind::String) source = Json::parse(source.str());
        const Json* list = &source;
        if (source.kind() == Json::Kind::Object) {
            data_set_ = source["data_set"].str();
            list = &source["changes"];
        }
        for (const auto& c : list->items()) changes_.push_back(c);
    }
    explicit Delta(const std::string& json) : Delta(Json::parse(json)) {}

    const std::string& data_set() const { return data_set_; }
    size_t size() const { return changes_.size(); }
    /// A alteração `i` (com as chaves de mestre já trocadas pelos `accept` anteriores).
    const Json& change(size_t i) const { return changes_[i]; }
    const std::vector<Json>& changes() const { return changes_; }
    const std::vector<Json>& results() const { return results_; }
    Json results_json() const { return Json::array(results_); }

    std::vector<Json> inserted(const std::string& data_set = "") const { return of("insert", data_set); }
    std::vector<Json> updated(const std::string& data_set = "") const { return of("update", data_set); }
    std::vector<Json> deleted(const std::string& data_set = "") const { return of("delete", data_set); }
    std::vector<Json> of_data_set(const std::string& data_set) const {
        std::vector<Json> out;
        for (const auto& c : changes_)
            if (c["data_set"].str() == data_set) out.push_back(c);
        return out;
    }

    /// Gravou; `values` são os que o banco definiu (a chave gerada).
    void accept(const Json& change, const Json& values = Json::object()) {
        long long ref = static_cast<long long>(change["ref"].number());
        for (auto& other : changes_) {
            const Json& link = other["master"];
            if (link.is_null() || static_cast<long long>(link["ref"].number()) != ref) continue;
            for (const auto& [detail, master] : link["fields"].members()) {
                const Json& value = values[master.str()];
                bool has = false;
                for (const auto& [k, v] : values.members()) has = has || k == master.str();
                if (!has) continue;
                for (const char* part : {"new", "key"}) {
                    Json map = other[part];
                    bool present = false;
                    for (const auto& [k, v] : map.members()) present = present || k == detail;
                    if (!present) continue;
                    map.set(detail, value);
                    other.set(part, map);
                }
            }
        }
        Json result = Json::object();
        result.set("ref", ref);
        if (!values.members().empty()) result.set("values", values);
        results_.push_back(result);
    }

    /// Não gravou: a mensagem vai para a tela e o registro continua pendente.
    void reject(const Json& change, const std::string& message) {
        Json result = Json::object();
        result.set("ref", static_cast<long long>(change["ref"].number()));
        result.set("error", message);
        results_.push_back(result);
    }

    /// O delta de exemplo dos selftests dos clientes (o mesmo em todas as linguagens).
    static constexpr const char* sample =
        "{\"data_set\":\"cdsPedidos\",\"changes\":["
        "{\"ref\":1,\"op\":\"insert\",\"data_set\":\"cdsPedidos\",\"key\":{\"id\":-1},\"new\":{\"id\":-1,\"cliente\":\"Joao\"}},"
        "{\"ref\":2,\"op\":\"insert\",\"data_set\":\"cdsItens\",\"key\":{\"id\":-2},\"new\":{\"id\":-2,\"pedido_id\":-1},"
        "\"master\":{\"data_set\":\"cdsPedidos\",\"ref\":1,\"fields\":{\"pedido_id\":\"id\"}}},"
        "{\"ref\":3,\"op\":\"update\",\"data_set\":\"cdsItens\",\"key\":{\"id\":10},\"old\":{\"produto\":\"A\"},\"new\":{\"produto\":\"B\"}},"
        "{\"ref\":4,\"op\":\"delete\",\"data_set\":\"cdsPedidos\",\"key\":{\"id\":7},\"old\":{\"id\":7}}]}";

    /// O texto que todos os clientes escrevem ao conferir o Delta com o `sample`.
    static std::string selftest() {
        Delta delta{std::string(sample)};
        delta.accept(delta.inserted("cdsPedidos")[0], {{"id", 501}});
        long long pedido = static_cast<long long>(delta.inserted("cdsItens")[0]["new"]["pedido_id"].number());
        return "delta " + std::to_string(delta.inserted().size()) + " " + std::to_string(delta.updated().size()) + " " +
               std::to_string(delta.deleted().size()) + " " + std::to_string(pedido) + " " + std::to_string(delta.results().size());
    }

private:
    std::vector<Json> of(const std::string& op, const std::string& data_set) const {
        std::vector<Json> out;
        for (const auto& c : changes_)
            if (c["op"].str() == op && (data_set.empty() || c["data_set"].str() == data_set)) out.push_back(c);
        return out;
    }

    std::string data_set_;
    std::vector<Json> changes_;
    std::vector<Json> results_;
};

}  // namespace aylis

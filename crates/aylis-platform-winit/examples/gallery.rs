//! Os componentes equivalentes aos do Vuetify, numa grade responsiva (`GridLayout`).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example gallery
//! cargo run -p aylis-platform-winit --example gallery -- --theme "Material 3 Dark"
//! ```

use aylis_controls::*;
use aylis_core::*;

/// Um cartão da galeria com o título e o componente.
fn card(
    ui: &mut Ui,
    grid: ComponentId,
    title: &str,
    height: f32,
    build: impl FnOnce(&mut Ui, ComponentId),
) {
    let panel = ui.add(Panel::default(), grid);
    ui.control_mut(panel)
        .unwrap()
        .set_size(10.0, height)
        .set_padding(12.0);
    let _ = ui.set_prop(panel, "style", "card");
    let label = ui.add(Label::new(title), panel);
    ui.control_mut(label)
        .unwrap()
        .set_align(Align::Top)
        .set_height(22.0)
        .set_margins([0.0, 0.0, 0.0, 6.0]);
    let _ = ui.set_prop(label, "style", "caption");
    build(ui, panel);
}

/// Um componente com o padrão e algumas propriedades trocadas.
fn with<T: Default>(f: impl FnOnce(&mut T)) -> T {
    let mut t = T::default();
    f(&mut t);
    t
}

fn client<T: Component>(ui: &mut Ui, parent: ComponentId, c: T) -> ComponentId {
    let id = ui.add(c, parent);
    ui.control_mut(id).unwrap().set_align(Align::Client);
    id
}

fn top<T: Component>(ui: &mut Ui, parent: ComponentId, c: T, h: f32) -> ComponentId {
    let id = ui.add(c, parent);
    ui.control_mut(id)
        .unwrap()
        .set_align(Align::Top)
        .set_height(h)
        .set_margins([0.0, 0.0, 0.0, 8.0]);
    id
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let args: Vec<String> = std::env::args().collect();
    if let Some(name) = args
        .iter()
        .position(|a| a == "--theme")
        .and_then(|i| args.get(i + 1))
    {
        if let Some(t) = aylis_core::themes::builtin(name) {
            ui.set_theme(t.clone());
        }
    }
    let form = ui.add_root(Form::new("aylis — componentes (Vuetify)", 1320.0, 1180.0));
    ui.set_name(form, "frmGaleria")?;
    let scroll = ui.add(ScrollBox::default(), form);
    ui.control_mut(scroll).unwrap().set_align(Align::Client);
    let grid = ui.add(
        with(|x: &mut GridLayout| {
            x.min_column_width = 300.0;
            x.gap = 14.0;
        }),
        scroll,
    );
    ui.control_mut(grid)
        .unwrap()
        .set_align(Align::Top)
        .set_height(2300.0)
        .set_padding(14.0);

    card(
        &mut ui,
        grid,
        "Edit: rótulo, máscara, regras, contador",
        230.0,
        |ui, p| {
            let mut e = Edit::new("");
            e.label = "Telefone".into();
            e.mask = "(##) #####-####".into();
            e.helper = "Com DDD".into();
            let e1 = top(ui, p, e, 54.0);
            let _ = ui.set_prop(e1, "text", "11987654321");
            let mut e2 = Edit::new("joao@");
            e2.label = "E-mail".into();
            e2.rules = "required;email".into();
            e2.error = "E-mail inválido".into();
            top(ui, p, e2, 54.0);
            let mut e3 = Edit::new("12,50");
            e3.label = "Preço".into();
            e3.prefix = "R$".into();
            e3.counter = true;
            e3.max_length = 10;
            top(ui, p, e3, 54.0);
        },
    );
    card(&mut ui, grid, "AutoComplete", 230.0, |ui, p| {
        let mut a = AutoComplete::default();
        a.base.label = "Cidade".into();
        top(ui, p, a, 36.0);
        let ac = top(ui, p, ColorEdit::default(), 36.0);
        let _ = ac;
        top(ui, p, FileInput::default(), 36.0);
    });
    card(&mut ui, grid, "Chip, ChipGroup", 230.0, |ui, p| {
        let flow = client(ui, p, FlowLayout::default());
        for (text, style, closable) in [
            ("Rust", "", false),
            ("Pascal", "outline", false),
            ("Pronto", "success", false),
            ("Atrasado", "danger", false),
            ("Filtro", "filter", false),
            ("Removível", "", true),
        ] {
            let mut c = Chip::new(text);
            c.closable = closable;
            c.checked = style == "filter";
            c.base.set_size(if closable { 116.0 } else { 96.0 }, 32.0);
            let id = ui.add(c, flow);
            let _ = ui.set_prop(id, "style", style);
        }
        let g = ui.add(
            with(|x: &mut ChipGroup| {
                x.multiple = true;
                x.selected = "Todos;Fechados".into();
            }),
            flow,
        );
        ui.control_mut(g).unwrap().set_size(300.0, 40.0);
    });
    card(
        &mut ui,
        grid,
        "Rating, RangeSlider, OtpInput",
        230.0,
        |ui, p| {
            top(
                ui,
                p,
                with(|x: &mut Rating| {
                    x.value = 3.5;
                    x.half = true;
                }),
                32.0,
            );
            top(ui, p, RangeSlider::default(), 32.0);
            top(
                ui,
                p,
                with(|x: &mut OtpInput| {
                    x.text = "482".into();
                }),
                46.0,
            );
        },
    );
    card(
        &mut ui,
        grid,
        "SplitButton, ProgressCircle, Kbd",
        230.0,
        |ui, p| {
            let row = top(ui, p, Layout::default(), 40.0);
            let b = ui.add(SplitButton::default(), row);
            ui.control_mut(b)
                .unwrap()
                .set_align(Align::Left)
                .set_width(160.0);
            let row2 = top(ui, p, Layout::default(), 60.0);
            for (v, ind) in [(72.0, false), (35.0, false), (0.0, true)] {
                let c = ui.add(
                    with(|x: &mut ProgressCircle| {
                        x.value = v;
                        x.indeterminate = ind;
                    }),
                    row2,
                );
                ui.control_mut(c)
                    .unwrap()
                    .set_align(Align::Left)
                    .set_width(60.0)
                    .set_margins([0.0, 0.0, 12.0, 0.0]);
            }
            let row3 = top(ui, p, Layout::default(), 28.0);
            for k in ["Ctrl", "Shift", "P"] {
                let l = ui.create("Label.kbd", Some(row3)).unwrap();
                let _ = ui.set_prop(l, "text", k);
                ui.control_mut(l)
                    .unwrap()
                    .set_align(Align::Left)
                    .set_width(48.0)
                    .set_margins([0.0, 0.0, 6.0, 0.0]);
            }
        },
    );
    card(&mut ui, grid, "Snackbar, SkeletonLoader", 230.0, |ui, p| {
        top(
            ui,
            p,
            with(|x: &mut SkeletonLoader| {
                x.layout = "list-item,text@2".into();
            }),
            110.0,
        );
        top(
            ui,
            p,
            with(|x: &mut Snackbar| {
                x.active = true;
                x.timeout = 0;
            }),
            48.0,
        );
    });
    card(&mut ui, grid, "Pagination, Stepper", 230.0, |ui, p| {
        top(
            ui,
            p,
            with(|x: &mut Pagination| {
                x.page = 6;
                x.length = 20;
            }),
            36.0,
        );
        top(ui, p, Stepper::default(), 64.0);
    });
    card(&mut ui, grid, "Timeline", 270.0, |ui, p| {
        client(ui, p, Timeline::default());
    });
    card(&mut ui, grid, "EmptyState", 270.0, |ui, p| {
        client(ui, p, EmptyState::default());
    });
    card(&mut ui, grid, "ColorPicker", 340.0, |ui, p| {
        let c = ui.add(ColorPicker::default(), p);
        ui.control_mut(c)
            .unwrap()
            .set_rect(12.0, 34.0, 260.0, 290.0);
    });
    card(
        &mut ui,
        grid,
        "Calendar: intervalo e eventos; mês",
        340.0,
        |ui, p| {
            let c = top(
                ui,
                p,
                with(|x: &mut Calendar| {
                    x.range = true;
                }),
                0.0,
            );
            let _ = ui.set_prop(c, "date", "2026-10-12");
            let _ = ui.set_prop(c, "end_date", "2026-10-16");
            let _ = ui.set_prop(c, "events", "2026-10-20;2026-10-25");
            ui.control_mut(c).unwrap().set_align(Align::Client);
        },
    );
    card(&mut ui, grid, "EventCalendar", 340.0, |ui, p| {
        let c = client(ui, p, EventCalendar::default());
        let _ = ui.set_prop(c, "month", "2026-10");
        let _ = ui.set_prop(c, "events", "2026-10-05|Reunião;2026-10-12|Feriado|danger;2026-10-21|Entrega|success;2026-10-21|Revisão;2026-10-28|Viagem|warning");
    });
    card(&mut ui, grid, "Carousel, AvatarGroup", 260.0, |ui, p| {
        top(ui, p, AvatarGroup::default(), 40.0);
        let car = client(ui, p, Carousel::default());
        for (i, color) in ["#3F51B5", "#009688", "#FF7043"].iter().enumerate() {
            let slide = ui.add(Rectangle::default(), car);
            ui.control_mut(slide).unwrap().set_align(Align::Client);
            let _ = ui.set_prop(slide, "fill", Color::from_hex(color).unwrap());
            let _ = ui.set_prop(slide, "radius", 8);
            let l = ui.add(Label::new(format!("Slide {}", i + 1)), slide);
            ui.control_mut(l)
                .unwrap()
                .set_align(Align::Center)
                .set_size(120.0, 30.0);
            let _ = ui.set_prop(l, "color", Color::WHITE);
            let _ = ui.set_prop(l, "h_align", "center");
        }
    });
    card(
        &mut ui,
        grid,
        "NavigationDrawer, BottomNavigation",
        260.0,
        |ui, p| {
            ui.add(
                with(|x: &mut NavigationDrawer| {
                    x.drawer_width = 190.0;
                }),
                p,
            );
            ui.add(BottomNavigation::default(), p);
            let d = ui.add(
                with(|x: &mut Divider| {
                    x.text = "conteúdo".into();
                }),
                p,
            );
            ui.control_mut(d)
                .unwrap()
                .set_align(Align::Top)
                .set_height(30.0);
        },
    );
    card(
        &mut ui,
        grid,
        "SpeedDial, Chart sparkline/heatmap",
        260.0,
        |ui, p| {
            let sd = ui.add(
                with(|x: &mut SpeedDial| {
                    x.open = true;
                }),
                p,
            );
            ui.control_mut(sd)
                .unwrap()
                .set_align(Align::Right)
                .set_width(150.0);
            let spark = ui.create("Chart.sparkline", Some(p)).unwrap();
            ui.control_mut(spark)
                .unwrap()
                .set_align(Align::Top)
                .set_height(56.0);
            let _ = ui.set_prop(spark, "series", "Vendas=4,6,5,9,7,11,10,14,12,16");
            let heat = ui.create("Chart.heatmap", Some(p)).unwrap();
            ui.control_mut(heat).unwrap().set_align(Align::Client);
            let _ = ui.set_prop(heat, "labels", "S;T;Q;Q;S;S;D");
            let _ = ui.set_prop(
                heat,
                "series",
                "Manhã=3,5,8,6,9,2,1|Tarde=6,9,12,10,14,4,2|Noite=2,3,4,5,8,9,7",
            );
        },
    );
    card(&mut ui, grid, "CommandPalette", 260.0, |ui, p| {
        client(
            ui,
            p,
            with(|x: &mut CommandPalette| {
                x.query = "sa".into();
            }),
        );
    });
    card(
        &mut ui,
        grid,
        "FileDropZone, BottomSheet",
        260.0,
        |ui, p| {
            let z = top(
                ui,
                p,
                with(|x: &mut FileDropZone| {
                    x.files = "C:\\fotos\\praia.png;C:\\docs\\contrato.pdf".into();
                }),
                150.0,
            );
            let _ = z;
            let sheet = ui.add(BottomSheet::default(), p);
            let _ = ui.set_prop(sheet, "sheet_height", 54.0);
        },
    );
    card(&mut ui, grid, "TimePicker (relógio)", 380.0, |ui, p| {
        let t = ui.add(TimePicker::new("15:30"), p);
        ui.control_mut(t).unwrap().set_rect(12.0, 34.0, 270.0, 330.0);
    });
    card(&mut ui, grid, "EventCalendar: semana", 380.0, |ui, p| {
        let c = client(ui, p, EventCalendar::default());
        let _ = ui.set_prop(c, "view", "week");
        let _ = ui.set_prop(c, "date", "2026-10-21");
        let _ = ui.set_prop(c, "start_hour", 8i64);
        let _ = ui.set_prop(c, "end_hour", 18i64);
        let _ = ui.set_prop(
            c,
            "events",
            "2026-10-19 09:00-10:30|Daily;2026-10-21 14:00-15:30|Reunião|success;2026-10-21 14:30-16:00|Revisão|warning;2026-10-23|Feriado|danger;2026-10-22 11:00-12:00|Almoço",
        );
    });
    card(&mut ui, grid, "Badge preso, destaque, rolagem lateral", 380.0, |ui, p| {
        let row = top(ui, p, Layout::default(), 56.0);
        for (icon, count) in [("bell", "3"), ("chat", "12"), ("send", "")] {
            let b = ui.add(Button::with_icon(icon), row);
            ui.control_mut(b).unwrap().set_align(Align::Left).set_width(44.0).set_margins([0.0, 6.0, 18.0, 6.0]);
            let _ = ui.set_prop(b, "style", "icon");
            let badge = ui.add(Badge::default(), row);
            let _ = ui.set_prop(badge, "target", PropValue::Component(Some(b)));
            let _ = ui.set_prop(badge, "text", count);
        }
        let l = top(ui, p, Label::new("Resultado: São Paulo, Santos e São José dos Campos"), 48.0);
        let _ = ui.set_prop(l, "wrap", true);
        let _ = ui.set_prop(l, "highlight", "sao");
        let g = top(ui, p, ChipGroup::default(), 40.0);
        let _ = ui.set_prop(g, "wrap", false);
        let _ = ui.set_prop(g, "items", "Todos;Rust;Pascal;Python;Go;Kotlin;Swift;TypeScript");
        let sb = top(ui, p, ScrollBox::default(), 70.0);
        let _ = ui.set_prop(sb, "horizontal", true);
        for i in 0..8 {
            let r = ui.add(Rectangle::default(), sb);
            ui.control_mut(r).unwrap().set_align(Align::Left).set_width(80.0).set_margins([0.0, 0.0, 8.0, 0.0]);
            let _ = ui.set_prop(r, "fill", Color::from_hex(["#3F51B5", "#009688", "#FF7043", "#8E24AA"][i % 4]).unwrap());
            let _ = ui.set_prop(r, "radius", 8);
        }
    });
    // `--end`: abre rolado até os últimos cartões.
    if args.iter().any(|a| a == "--end") {
        ui.layout();
        ui.control_mut(scroll).unwrap().set_scroll_offset(PointF::new(0.0, 1120.0));
    }
    aylis_platform_winit::run(ui, form)
}

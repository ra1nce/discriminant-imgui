use imgui::*;

mod support;


struct Discriminant {
    d: f32,
    x1: f32,
    x2: f32
}


fn main() {
    let width_window: f32 = 400.0;
    let height_window: f32 = 350.0;
    
    let mut a = 1.0;
    let mut b = -4.0;
    let mut c = -5.0;
    let mut discriminant = Discriminant{d: 36.0, x1: 5.0, x2: -1.0};

    let system = support::init("Discriminant-imgui!", width_window, height_window,
    support::SystemColor { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 });
    

    system.main_loop(move |_, ui| {
        Window::new(file!())
            .position([0.0, 0.0], Condition::FirstUseEver)
            .size([width_window, height_window], Condition::FirstUseEver)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .draw_background(false)
            .build(ui, || {
                let mut info = String::new();

                if discriminant.d == 0.0 {
                    info = format!("D = {}\nx1 = {}", discriminant.d, discriminant.x1)
                } else if discriminant.d > 0.0 {
                    info = format!("D = {}\nx1 = {}\nx2 = {}", discriminant.d, discriminant.x1, discriminant.x2)
                } else if discriminant.d < 0.0 {
                    info = format!("D = {}\nNo x1/x2", discriminant.d)
                }

                ui.text(format!("{}x2+{}x+{}=0", a, b, c));

                ui.separator();

                if ui.input_float("A", &mut a).build() {
                    discriminant = get_discriminant(a, b, c);
                }
                if ui.input_float("B", &mut b).build() {
                    discriminant = get_discriminant(a, b, c);
                }
                if ui.input_float("C", &mut c).build() {
                    discriminant = get_discriminant(a, b, c);
                }

                ui.separator();

                ui.text(info);

            });
    });
}


fn get_discriminant(a: f32, b: f32, c: f32) -> Discriminant{
    // b2 - 4ac
    let d =  b * b - 4.0 * a * c;

    if d > 0.0 {
        Discriminant {
            d: d, 
            x1: (-b + d.sqrt()) / 2.0 * a, 
            x2: (-b - d.sqrt()) / 2.0 * a
        }
    } else if d == 0.0 {
        Discriminant {
            d: d, 
            x1: -b / (2.0 * a), 
            x2: 0.0
        }
    } else {
        Discriminant {
            d: d, 
            x1: 0.0, 
            x2: 0.0
        }
    }
    
}
use eframe::egui; 
use rand::Rng; 
use rand::thread_rng;

fn generator() -> String {
    let mut rng = thread_rng(); 


    let min = 0x21; // 33 em decimal ('!')
    let max = 0x7E; // 126 em decimal ('~')

    let mut resultado = String::new();

    for _ in 0..15 {
        // Generate random value between [0x21, 0x7E]
        let ascii_code = rng.gen_range(min..=max);


        let chara = char::from(ascii_code);


        resultado.push(chara);
    }

    resultado
}
struct MyApp {
    my_password: String,
}

impl MyApp {
    fn new() -> Self {
        MyApp {
            my_password
            : generator(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {



        egui::CentralPanel::default().show(ctx, |ui| {


            ui.heading("PassGen");

            ui.horizontal(|ui| {
                ui.label("Your Password:");
                ui.text_edit_singleline(&mut self.my_password)


            });

            
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {


    eframe::run_native(
        "PassGen - Generate your Security",  
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),  

    )

}

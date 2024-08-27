slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    // Use a shared mutable reference to track the current formula
    let is_fahrenheit_to_celsius = std::rc::Rc::new(std::cell::RefCell::new(true));

    ui.on_convert_fahrenheit({
        let ui_handle = ui.as_weak();
        let is_fahrenheit_to_celsius = is_fahrenheit_to_celsius.clone();
        move |string| {
            let ui = ui_handle.unwrap();
            let trimmed = string.trim();

            match trimmed.parse::<f64>() {
                Ok(value) => {
                    let result = if *is_fahrenheit_to_celsius.borrow() {
                        let celsius = (value - 32.0) * 5.0 / 9.0;
                        format!("Celsius: {:.2}", celsius)
                    } else {
                        let fahrenheit = (value * 9.0 / 5.0) + 32.0;
                        format!("Fahrenheit: {:.2}", fahrenheit)
                    };
                    ui.set_results(result.into());
                }
                Err(_) => {
                    ui.set_results("Invalid input. Please enter a number.".into());
                }
            }
        }
    });

    ui.on_change_formula({
        let ui_handle = ui.as_weak();
        let is_fahrenheit_to_celsius = is_fahrenheit_to_celsius.clone();
        move || {
            let ui = ui_handle.unwrap();
            let mut formula = is_fahrenheit_to_celsius.borrow_mut();
            *formula = !*formula; // Toggle the formula

            if *formula {
                ui.set_current_formula("Fahrenheit to Celsius".into());
                ui.set_window_title("Fahrenheit to Celsius Converter".into());
                ui.set_input_placeholder("F".into());
            } else {
                ui.set_current_formula("Celsius to Fahrenheit".into());
                ui.set_window_title("Celsius to Fahrenheit Converter".into());
                ui.set_input_placeholder("C".into());
            }
        }
    });

    ui.run()
}

use std::io;

fn main() {
  // Get user input for height
  let mut height_str = String::new();
  println!("Enter your height in centimeters (cm):");
  io::stdin().read_line(&mut height_str).expect("Failed to read height");

  // Convert height string to a float
  let height: f32 = match height_str.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid input. Please enter a number.");
      return;
    }
  };

  // Get user input for weight
  let mut weight_str = String::new();
  println!("Enter your weight in kilograms (kg):");
  io::stdin().read_line(&mut weight_str).expect("Failed to read weight");

  // Convert weight string to a float
  let weight: f32 = match weight_str.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid input. Please enter a number.");
      return;
    }
  };

  // Calculate BMI
  let bmi = calculate_bmi(height, weight);

  // Print BMI and interpretation
  println!("Your BMI is: {:.2}", bmi);
  println!("{}", interpret_bmi(bmi));
}

// Function to calculate BMI
fn calculate_bmi(height: f32, weight: f32) -> f32 {
  weight / (height / 100.0 * height / 100.0)
}

// Function to interpret BMI based on value
fn interpret_bmi(bmi: f32) -> String {
  let interpretation: String;
  if bmi < 18.5 {
    interpretation = "Underweight".to_string();
  } else if bmi < 25.0 {
    interpretation = "Normal weight".to_string();
  } else if bmi < 30.0 {
    interpretation = "Overweight".to_string();
  } else {
    interpretation = "Obese".to_string();
  }
  interpretation
}

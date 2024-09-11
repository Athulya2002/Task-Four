use chrono::Local; // Import chrono for working with dates and times

fn main() {
    let name = "Your Name"; // Replace with your name
    let current_time = Local::now(); // Get the current time

    println!("Hello {}, right now the time is {}", name, current_time.format("%Y-%m-%d %H:%M:%S"));
}

 

fn main() {    
    let count = [
        "uno", "dos", "tres", "cuatro", "cinco",
        "seis", "siete", "ocho", "nueve", "diez",
        "once", "doce", "trece", "catorce", "quince",
        "dieciséis", "diecisiete", "dieciocho", "diecinueve", "viente",
        "vientiuno", "vientidos", "vientitrés", "vienticuatro", "vienticinco", 
        "vientiséis", "vientisiete", "vientiocho", "vientinueve", "treinta", 
        "treinta y uno", "treinta y dos", "treinta y tres", "treinta y cuatro", "treinta y cinco", 
        "treinta y seis", "treinta y siete", "treinta y ocho", "treinta y nueve"
    ];
    let error_msg = format!("The one integer number in [0, {}] expected", count.len());
    let s = std::env::args().nth(1).unwrap_or("4".to_string());
    
    match s.trim().parse::<usize>() {
        Ok(n) =>
            if n <= count.len() {
                for i in 0..n { println!("{}", count[i]) }
            } else { println!("{}", error_msg) },
        Err(_) =>
            println!("{}", error_msg)
    }
}

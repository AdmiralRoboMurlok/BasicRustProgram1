use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let var = rng.gen_range(0..3);

    let mut rps;

    if var == 0{
        rps = "kamień";
    } else if var == 1{
        rps = "papier";
    } else if var == 2{
        rps = "nożyce";
    } else {
        rps = "error"
    }

    let mut ruch_gracz = read_string().trim().to_string();

    println!("{}", rps);

    if ruch_gracz == "kamień" && rps == "nożyce" {
        println!("Wygrałeś")
    } else if ruch_gracz == "papier" && rps == "kamień"{
        println!("Wygrałeś")
    } else if ruch_gracz == "nożyce" && rps == "papier" {
        println!("Wygrałeś")
    } else if ruch_gracz == "nożyce" && rps == "kamień"{
        println!("Przegrałeś")
    } else if ruch_gracz == "kamień" &&  rps == "papier"{
        println!("Przegrałeś")
    } else if ruch_gracz == "papier" && rps == "nożyce"{
        println!("Przegrałeś")
    } else {
        println!("Remis")
    }
}

fn read_string() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Can't read user input");

    input
}
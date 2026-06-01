// NEON_SHADOW_RPG v0.1.0
// [CYBERPUNK PROTOCOL ACTIVE] - RUST CORE BOOTING IN THE UNDERGRID...

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Runner {
    name: String,
    health: i32,
    chrome: i32,  // Cyberware level
    creds: i32,
    level: i32,
}

impl Runner {
    fn new(name: &str) -> Self {
        Runner {
            name: name.to_string(),
            health: 100,
            chrome: 25,
            creds: 500,
            level: 1,
        }
    }

    fn status(&self) {
        println!("\n\x1b[31m[NEURAL LINK: {}]\x1b[0m", self.name);
        println!("\x1b[32mVITALS: {} | CHROME: {} | CREDS: {}¥ | LVL: {}\x1b[0m", 
                self.health, self.chrome, self.creds, self.level);
    }
}

fn glitch_print(text: &str, delay: u64) {
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay));
    }
    println!();
}

fn main() {
    glitch_print("
    ███╗   ██╗███████╗ ██████╗ ███╗   ██╗    ███████╗██╗  ██╗ █████╗ ██████╗  ██████╗ ██╗    ██╗
    ████╗  ██║██╔════╝██╔═══██╗████╗  ██║    ██╔════╝██║  ██║██╔══██╗██╔══██╗██╔═══██╗██║    ██║
    ██╔██╗ ██║█████╗  ██║   ██║██╔██╗ ██║    ███████╗███████║███████║██████╔╝██║   ██║██║ █╗ ██║
    ██║╚██╗██║██╔══╝  ██║   ██║██║╚██╗██║    ╚════██║██╔══██║██╔══██║██╔══██╗██║   ██║██║███╗██║
    ██║ ╚████║███████╗╚██████╔╝██║ ╚████║    ███████║██║  ██║██║  ██║██║  ██║╚██████╔╝╚███╔███╔╝
    ╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝  ╚══╝╚══╝ 
    ", 5);

    glitch_print("\n\x1b[33m>>> ENTERING THE NEON SHADOWS... YOUR DECK IS ONLINE <<<\x1b[0m\n", 20);

    print!("\x1b[36mCHOOSE YOUR HANDLE, RUNNER: \x1b[0m");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    let mut player = Runner::new(name);

    glitch_print(&format!("\nWELCOME TO THE GRID, {}...", player.name), 30);
    
    loop {
        player.status();
        
        println!("\n\x1b[35m[MAINFRAME MENU]\x1b[0m");
        println!("1. JACK INTO THE STREETS (EXPLORE NIGHT CITY)");
        println!("2. HIT A FIXER (QUESTS)");
        println!("3. CHROME UP (UPGRADES)");
        println!("4. RUN THE MATRIX (COMBAT SIM)");
        println!("5. FLATLINE (EXIT THE NET)");
        
        print!("\x1b[36m> \x1b[0m");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => explore_streets(&mut player),
            "2" => fixer_job(&mut player),
            "3" => chrome_shop(&mut player),
            "4" => matrix_run(&mut player),
            "5" => {
                glitch_print("\n\x1b[31mDISCONNECTING FROM THE GRID... STAY CHROME, RUNNER.\x1b[0m", 10);
                break;
            }
            _ => glitch_print("\x1b[33m[ERROR] INVALID PROTOCOL. TRY AGAIN.\x1b[0m", 5),
        }
        
        // Random chrome decay / events
        if rand::random::<f32>() > 0.7 {
            player.health -= 5;
            glitch_print("\x1b[31m[WARNING] BLACK ICE DETECTED. MINOR SYSTEM DAMAGE.\x1b[0m", 10);
        }
        
        if player.health <= 0 {
            glitch_print("\n\x1b[31m>>> FLATLINED. GAME OVER, RUNNER. BETTER LUCK IN THE NEXT LIFECYCLE <<<\x1b[0m", 15);
            break;
        }
    }
}

fn explore_streets(player: &mut Runner) {
    glitch_print("\n\x1b[32mYou step into the rain-slicked streets of Night City...\x1b[0m", 20);
    let events = vec![
        "A street vendor offers you synth-meat. +50¥",
        "Gang ambush! You escape but take hits.",
        "Found a data shard with corporate dirt. +100 creds",
    ];
    let event = events[rand::random::<usize>() % events.len()];
    glitch_print(&format!("[EVENT] {}", event), 15);
    
    if event.contains("50") {
        player.creds += 50;
    } else if event.contains("100") {
        player.creds += 100;
    } else {
        player.health -= 15;
    }
}

fn fixer_job(player: &mut Runner) {
    glitch_print("\n\x1b[35mA FIXER CONTACTS YOU ON A BURNER...\x1b[0m", 20);
    println!("\"Need a decker for a quick corp run. You in?\"");
    
    if player.creds > 200 {
        glitch_print("MISSION COMPLETE. +300¥ | +1 LEVEL", 10);
        player.creds += 300;
        player.level += 1;
        player.chrome += 10;
    } else {
        glitch_print("\x1b[31mNOT ENOUGH CREDS FOR THIS RUN.\x1b[0m", 10);
    }
}

fn chrome_shop(player: &mut Runner) {
    glitch_print("\n\x1b[33mWELCOME TO THE RIPPERDOC CLINIC\x1b[0m", 15);
    println!("1. Kerenzikov Reflex (+20 chrome | 400¥)");
    println!("2. Subdermal Armor (+30 health | 600¥)");
    
    print!("> ");
    io::stdout().flush().unwrap();
    let mut ch = String::new();
    io::stdin().read_line(&mut ch).unwrap();
    
    match ch.trim() {
        "1" if player.creds >= 400 => {
            player.chrome += 20;
            player.creds -= 400;
            glitch_print("REFLEX BOOSTERS INSTALLED. YOU MOVE LIKE LIGHTNING.", 10);
        }
        "2" if player.creds >= 600 => {
            player.health += 30;
            player.creds -= 600;
            glitch_print("ARMOR GRAFTED. YOU FEEL INVINCIBLE.", 10);
        }
        _ => glitch_print("NOT ENOUGH CREDS OR INVALID INPUT.", 10),
    }
}

fn matrix_run(player: &mut Runner) {
    glitch_print("\n\x1b[36mENTERING THE MATRIX... ICE IS THICK TONIGHT.\x1b[0m", 20);
    
    let mut enemy_health = 80;
    while enemy_health > 0 && player.health > 0 {
        println!("\n[MATRIX COMBAT]");
        println!("1. SLASH (Basic attack)");
        println!("2. OVERCLOCK (Risky but powerful)");
        
        print!("> ");
        io::stdout().flush().unwrap();
        let mut action = String::new();
        io::stdin().read_line(&mut action).unwrap();
        
        match action.trim() {
            "1" => {
                let dmg = 15 + (player.chrome / 5);
                enemy_health -= dmg;
                glitch_print(&format!("DECK SLASH HITS FOR {} DMG!", dmg), 5);
            }
            "2" => {
                let dmg = 35 + (player.chrome / 3);
                enemy_health -= dmg;
                player.health -= 10; // Risk
                glitch_print(&format!("OVERCLOCKED! {} DMG BUT FEEDBACK HITS!", dmg), 5);
            }
            _ => {}
        }
        
        // Enemy counter
        if enemy_health > 0 {
            let edmg = 12;
            player.health -= edmg;
            glitch_print(&format!("[ICE] COUNTERATTACK FOR {}!", edmg), 5);
        }
    }
    
    if enemy_health <= 0 {
        glitch_print("\x1b[32mICE SHATTERED. LOOT ACQUIRED: +250¥\x1b[0m", 15);
        player.creds += 250;
    }
}

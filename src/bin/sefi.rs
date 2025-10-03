// Sefi CLI for Phase 1

use sefi::{ConceptPacket, Polarity, Tempo};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "emit" => emit_command(&args[2..]),
        "status" => status_command(),
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Sefi v0.3 - Semantic Field Blackboard (N-D Primary)");
    println!();
    println!("Usage:");
    println!("  sefi emit <phrase> [--amp <0.0-1.0>] [--tempo fast|slow|urgent]");
    println!("  sefi status");
    println!();
    println!("Examples:");
    println!("  sefi emit \"memory safety\" --amp 0.9 --tempo fast");
    println!("  sefi emit \"consensus pattern\" --tempo slow");
    println!("  sefi status");
}

fn emit_command(args: &[String]) {
    if args.is_empty() {
        println!("Error: phrase required");
        return;
    }

    let phrase = &args[0];
    let mut amp = 0.5;
    let mut tempo = Tempo::Slow;

    // Parse optional args
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--amp" => {
                if i + 1 < args.len() {
                    amp = args[i + 1].parse().unwrap_or(0.5);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--tempo" => {
                if i + 1 < args.len() {
                    tempo = match args[i + 1].as_str() {
                        "fast" => Tempo::Fast,
                        "slow" => Tempo::Slow,
                        "urgent" => Tempo::Urgent,
                        _ => Tempo::Slow,
                    };
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let packet = ConceptPacket {
        phrase: phrase.clone(),
        amp,
        sigma: 1.0,
        polarity: Polarity::Attract,
        tempo,
        provenance: "cli".to_string(),
        agent_id: "human".to_string(),
        rationale_hash: format!("cli_{}", now),
        timestamp: now,
    };

    println!("Emitted packet:");
    println!("  phrase: {}", packet.phrase);
    println!("  amp: {}", packet.amp);
    println!("  tempo: {:?} (τ={}s)", packet.tempo, packet.tempo.tau());
    println!("  timestamp: {}", packet.timestamp);
    println!();
    println!("TODO: Store in ledger and trigger clustering");
}

fn status_command() {
    println!("Sefi Status:");
    println!("  Version: 0.3 (N-D Primary)");
    println!("  Mode: Phase 1 - Minimal Loop");
    println!();
    println!("TODO: Show ledger size, active basins, etc.");
}

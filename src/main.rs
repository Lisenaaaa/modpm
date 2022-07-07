use std::collections::HashMap;

use clap::{arg, Command};
use modpm::data_structs::MpmMod;
use modpm::PolyMC;

fn cli() -> Command<'static> {
    Command::new("modpm")
        .about("A Minecraft mod package manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("query")
                .about("Queries a mod")
                .arg(arg!(<MOD> "The mod to query."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("download")
                .about("Downloads a mod")
                .arg(arg!(<MOD> "The mod to download."))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("polymc").about("testing lmao"))
        .subcommand(Command::new("test").about("even more testing"))
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("query", sub_matches)) => {
            let mmod = sub_matches.get_one::<String>("MOD").expect("required");

            query_mod(&mmod[..]).await;
        }
        Some(("download", _sub_matches)) => {
            /*
                let mmod = sub_matches.get_one::<String>("MOD").expect("required");
                let queried_mod = Mod::query(mmod).await.unwrap();

                println!(
                    "I found {}, with the ID {}.",
                    queried_mod.name, queried_mod.id
                );

                let instances = PolyMC::get_instances().expect("Couldn't get PolyMC instances.");
                for instance in &instances {
                    println!(
                        "{}: {} - {} {}",
                        instance.id,
                        ansi_term::Color::Blue.paint(&instance.name),
                        ansi_term::Color::Purple.paint(&instance.modloader),
                        ansi_term::Color::Green.paint(&instance.game_version)
                    );
                }

                let instance_id = ask_user("What instance do you want to download this mod to? ");

                let instance = instances
                    .into_iter()
                    .find(|i| i.id.to_string() == instance_id)
                    .expect("Couldn't find that instance.");

                println!(
                    "{} - {} {}",
                    instance.name, instance.modloader, instance.game_version
                );

                queried_mod.download(instance).await.unwrap();
            */
        }
        Some(("polymc", _)) => {
            let instances = PolyMC::get_instances().unwrap();

            for instance in instances {
                println!(
                    "{}: {} - {} {}",
                    instance.id, instance.name, instance.modloader, instance.game_version
                );
            }
        }
        Some(("test", _)) => {
            println!("{}", PolyMC::get_directory());
        }
        _ => unreachable!(),
    }
}

async fn query_mod(mmod: &str) {
    let mod_data = MpmMod::new(mmod).await.expect("Couldn't get mod.");
    println!(
        "I found {}, which is licensed under {}, and located at {}",
        ansi_term::Color::Green.paint(&mod_data.title),
        ansi_term::Color::Green.paint(&mod_data.license.name),
        ansi_term::Color::RGB(255, 165, 0).paint(&mod_data.source_url)
    );
    println!("{}", mod_data.description);

    let mut members: HashMap<String, Vec<String>> = HashMap::new();
    members.insert("Owner".to_string(), vec![]);

    for member in mod_data.members {
        let _entry = match members.entry(member.role) {
            std::collections::hash_map::Entry::Vacant(role) => {
                let new_value = vec![member.user.name.unwrap_or(member.user.username)];
                role.insert(new_value);
            }
            std::collections::hash_map::Entry::Occupied(mut role) => {
                role.get_mut()
                    .push(member.user.name.unwrap_or(member.user.username));
            }
        };
    }

    for role in members.keys() {
        let people = members.get(role).unwrap();
        println!("{}: {}", role, people.join(", "));
    }
}

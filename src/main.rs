use figlet_rs::FIGfont;
use serde_json::Value;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use sysinfo::System;

#[tokio::main]
async fn main() {
    let standard_font = FIGfont::standard().expect("Failed to load standard font");
    let figure = standard_font
        .convert("setup-mc")
        .expect("Failed to render text");

    println!("{}", figure);
    server_dir().await;
    download_software(select_software().await).await;
    write_eula().await;
    create_start_script().await;
}

async fn server_dir() {
    print!("Enter a directory to store the server: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let dir = input.trim();
    if std::path::Path::new(dir).exists() {
        print!("Directory already exists. Continue? (y/n): ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input != "y" {
            std::process::exit(1);
        }
    }
    std::fs::create_dir_all(dir).expect("Failed to create directory");
    std::env::set_current_dir(dir).expect("Failed to set directory");
}

async fn select_software() -> i32 {
    println!("1. PurpurMC");
    println!("2. PaperMC");
    println!("3. Vanilla");
    println!("4. Fabric");
    println!("0. Exit");
    print!("Select a server type: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    return input.parse::<i32>().unwrap();
}

async fn download_software(software: i32) {
    match software {
        1 => {
            // get versions
            let response = reqwest::get("https://api.purpurmc.org/v2/purpur")
                .await
                .expect("Failed to get versions");
            let versions: Value = response.json().await.expect("Failed to parse JSON");
            // version selector
            println!("Available versions for PaperMC:");
            for (_, version) in versions["versions"].as_array().unwrap().iter().enumerate() {
                print!("{}\t", version.to_string().replace("\"", ""));
            }
            println!();
            print!("Enter a version: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let version = input.trim();
            // make sure the version is valid
            if versions["versions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|&x| x.to_string().replace("\"", "") == version)
                .is_none()
            {
                println!("Invalid version.");
                return;
            }
            // download
            let response = reqwest::get(format!(
                "https://api.purpurmc.org/v2/purpur/{}/latest/download",
                version
            ))
            .await
            .expect("Failed to get download link");
            let bytes = response
                .bytes()
                .await
                .expect("Failed to get download bytes");
            std::fs::write("server.jar", bytes).expect("Failed to write file");
            info_file(version, 1);
            println!("Downloaded PurpurMC {}.", version);
        }
        2 => {
            // get versions
            let response = reqwest::get("https://api.papermc.io/v2/projects/paper")
                .await
                .expect("Failed to get versions");
            let versions: Value = response.json().await.expect("Failed to parse JSON");
            // version selector
            println!("Available versions for PaperMC:");
            for (_, version) in versions["versions"].as_array().unwrap().iter().enumerate() {
                print!("{}\t", version.to_string().replace("\"", ""));
            }
            println!();
            print!("Enter a version: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let version = input.trim();
            // make sure the version is valid
            if versions["versions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|&x| x.to_string().replace("\"", "") == version)
                .is_none()
            {
                println!("Invalid version.");
                return;
            }
            // download
            let response = reqwest::get(format!(
                "https://api.papermc.io/v2/projects/paper/{}/latest/download",
                version
            ))
            .await
            .expect("Failed to get download link");
            let bytes = response
                .bytes()
                .await
                .expect("Failed to get download bytes");
            std::fs::write("server.jar", bytes).expect("Failed to write file");
            println!("Downloaded PaperMC {}.", version);
            info_file(version, 2);
        }
        3 => {
            // download the launcher manifest https://launchermeta.mojang.com/mc/game/version_manifest.json
            let response =
                reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
                    .await
                    .expect("Failed to get version manifest");
            let versions: Value = response.json().await.expect("Failed to parse JSON");
            for (_, version) in versions["versions"]
                .as_array()
                .unwrap()
                .iter()
                .rev()
                .enumerate()
            {
                if version["type"].to_string().replace("\"", "") == "release" {
                    print!("{}\t", version["id"].to_string().replace("\"", ""));
                } else {
                }
            }
            println!();
            print!("Enter a version or press s to show snapshot versions: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let version = input.trim();
            // make sure the version is valid
            if versions["versions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|&x| x["id"].to_string().replace("\"", "") == version)
                .is_none()
            {
                if version == "s" {
                    for (_, version) in versions["versions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .rev()
                        .enumerate()
                    {
                        if version["type"].to_string().replace("\"", "") == "snapshot" {
                            print!("{}\t", version["id"].to_string().replace("\"", ""));
                        }
                    }
                    println!();
                    print!("Enter a version: ");
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let version = input.trim();
                    if versions["versions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|&x| x["id"].to_string().replace("\"", "") == version)
                        .is_none()
                    {
                        println!("Invalid version.");
                        return;
                    }
                } else {
                    println!("Invalid version.");
                    return;
                }
                println!("Invalid version.");
                return;
            }
            // download
            let response = reqwest::get(
                versions["versions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|&x| x["id"].to_string().replace("\"", "") == version)
                    .unwrap()["url"]
                    .to_string()
                    .replace("\"", ""),
            )
            .await
            .expect("Failed to get download link");
            let version_manifest: Value = response.json().await.expect("Failed to parse JSON");
            let response = reqwest::get(
                version_manifest["downloads"]["server"]["url"]
                    .to_string()
                    .replace("\"", ""),
            )
            .await
            .expect("Failed to get download link");
            let bytes = response
                .bytes()
                .await
                .expect("Failed to get download bytes");
            std::fs::write("server.jar", bytes).expect("Failed to write file");
            info_file(version, 3);
            println!("Downloaded Vanilla {}.", version);
        }
        4 => {
            // get the game version manifest at https://meta.fabricmc.net/v2/versions/game
            let response = reqwest::get("https://meta.fabricmc.net/v2/versions/game")
                .await
                .expect("Failed to get version manifest");
            let versions: Value = response.json().await.expect("Failed to parse JSON");
            // version selector
            println!("Available versions for Fabric:");
            for (_, version) in versions.as_array().unwrap().iter().enumerate() {
                if version["stable"].as_bool().unwrap() {
                    print!("{}\t", version["version"].to_string().replace("\"", ""));
                }
            }
            println!();
            print!("Enter a version: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let version = input.trim();
            // make sure the version is valid
            if versions
                .as_array()
                .unwrap()
                .iter()
                .find(|&x| x["version"].to_string().replace("\"", "") == version)
                .is_none()
            {
                println!("Invalid version.");
                return;
            }
            // download
            let response = reqwest::get(format!(
                "https://meta.fabricmc.net/v2/versions/loader/{}",
                version
            ))
            .await
            .expect("Failed to get download link");
            let loader_manifest: Value = response.json().await.expect("Failed to parse JSON");
            // get the latest loader version
            let loader_version = loader_manifest["loader"]["version"]
                .to_string()
                .replace("\"", "");
            // get /v2/versions/installer and the first entry in the array with stable true's version number
            let response = reqwest::get("https://meta.fabricmc.net/v2/versions/installer")
                .await
                .expect("Failed to get version manifest");
            let installer_versions: Value = response.json().await.expect("Failed to parse JSON");
            let installer_version = installer_versions
                .as_array()
                .unwrap()
                .iter()
                .find(|&x| x["stable"].as_bool().unwrap())
                .unwrap()["version"]
                .to_string()
                .replace("\"", "");
            // download the server from /v2/versions/loader/gameversion/{loader.version}/installer_version/server/jar
            let response = reqwest::get(format!(
                "https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}",
                version, loader_version, installer_version
            ))
            .await
            .expect("Failed to get download link");
            let bytes = response
                .bytes()
                .await
                .expect("Failed to get download bytes");
            std::fs::write("server.jar", bytes).expect("Failed to write file");
            info_file(version, 4);
            println!("Downloaded Fabric {}.", version);
        }
        _ => {
            std::process::exit(1);
        }
    }
    fn info_file(game_version: &str, software: i32) {
        let mut file = std::fs::File::create("info.txt").expect("Failed to create file");
        file.write_all(
            format!("Game Version: {}\nSoftware: {}", game_version, software).as_bytes(),
        )
        .expect("Failed to write file");
    }
}

async fn write_eula() {
    print!("Do you agree to the Minecraft EULA? (y/n): ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input != "y" {
        std::process::exit(1);
    }
    std::fs::write("eula.txt", "eula=true").expect("Failed to write file");
    println!("Placed eula.txt in the server directory.");
}

async fn create_start_script() {
    let info = std::fs::read_to_string("info.txt").expect("Failed to read file");
    let software = info.split("\n").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()[1];
    let s = System::new_all();
    println!("Memory amount selection:");
    println!(
        "Your system has {} GB of RAM. The server will need at least 1 GB of RAM to run, and you should leave at least 1 GB overhead for the system (i.e. you shouldn't allocate more than {} GB of RAM to the server).",
        s.total_memory() / 1024 / 1024 / 1024,
        s.total_memory() / 1024 / 1024 / 1024 - 1
    );
    print!("Enter the amount of memory to allocate to the server in GB: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let memory = input.trim();
    if (memory.parse::<u64>().unwrap()) > s.total_memory() / 1024 / 1024 / 1024 {
        println!("{}, {}, This may be an invalid memory amount for your system. Would you like to continue? (y/n): ", memory.parse::<u64>().unwrap(), s.total_memory() / 1024 / 1024 / 1024);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input != "y" {
            std::process::exit(1);
        }
    }
    let mut filename = "start.sh";
    if std::env::consts::OS == "windows" {
        filename = "start.bat";
    }
    let mut args = vec![
        "-Xmx".to_string() + memory + "G",
        "-Xms512M".to_string(),
        "-jar".to_string(),
        "server.jar".to_string(),
        "nogui".to_string(),
    ];
    // prompt the user about the aikars flags if they are using softwares 1 or 2
    if software == "1" || software == "2" {
        print!("Would you like to use Aikar's flags? (y/n): ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "y" {
            args.insert(2, "-XX:+UseG1GC".to_string());
            args.insert(3, "-XX:+ParallelRefProcEnabled".to_string());
            args.insert(4, "-XX:MaxGCPauseMillis=200".to_string());
            args.insert(5, "-XX:+UnlockExperimentalVMOptions".to_string());
            args.insert(6, "-XX:+DisableExplicitGC".to_string());
            args.insert(7, "-XX:+AlwaysPreTouch".to_string());
            args.insert(8, "-XX:G1NewSizePercent=30".to_string());
            args.insert(9, "-XX:G1MaxNewSizePercent=40".to_string());
            args.insert(10, "-XX:G1HeapRegionSize=8M".to_string());
            args.insert(11, "-XX:G1ReservePercent=20".to_string());
            args.insert(12, "-XX:G1HeapWastePercent=5".to_string());
            args.insert(13, "-XX:G1MixedGCCountTarget=4".to_string());
            args.insert(14, "-XX:InitiatingHeapOccupancyPercent=15".to_string());
            args.insert(15, "-XX:G1MixedGCLiveThresholdPercent=90".to_string());
            args.insert(16, "-XX:G1RSetUpdatingPauseTimePercent=5".to_string());
            args.insert(17, "-XX:SurvivorRatio=32".to_string());
            args.insert(18, "-XX:+PerfDisableSharedMem".to_string());
            args.insert(19, "-XX:MaxTenuringThreshold=1".to_string());
            args.insert(
                19,
                "-Dusing.aikars.flags=https://mcflags.emc.gs".to_string(),
            );
        }
    }
    // if we're on a unix system, we need to add the shebang and make the file executable
    if std::env::consts::OS != "windows" {
        // ask the user if they'd like to use tmux
        print!("Would you like to use tmux? (y/n): ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "y" {
            std::fs::write(
                filename,
                format!(
                    "#!/bin/bash\ntmux new-session -d -s mc 'java {}\n'\n",
                    args.join(" ")
                )
                .as_bytes(),
            )
            .expect("Failed to write file");
            #[cfg(unix)]
            std::fs::set_permissions(filename, std::fs::Permissions::from_mode(0o755))
                .expect("Failed to set permissions");
            println!("Created start script with tmux.");
            return;
        } else {
            std::fs::write(
                filename,
                format!("#!/bin/bash\njava {}\n", args.join(" ")).as_bytes(),
            )
            .expect("Failed to write file");
        }
        #[cfg(unix)]
        std::fs::set_permissions(filename, std::fs::Permissions::from_mode(0o755))
            .expect("Failed to set permissions");
    } else {
        std::fs::write(filename, format!("java {}\n", args.join(" ")).as_bytes())
            .expect("Failed to write file");
    }
    println!("Created start script.");
}

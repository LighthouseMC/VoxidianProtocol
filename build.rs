#![feature(
    decl_macro,
    exit_status_error
)]


use std::process::Command;
use std::ffi::OsString;
use std::io::Write;
use std::{ env, fs };


macro ver {
    ( MINECRAFT    ) => { "1.21.4"                                                },
    ( LOOM_BUILD   ) => { "2"                                                     },
    ( MAPPINGS     ) => { concat!( ver!(MINECRAFT), "+build.", ver!(LOOM_BUILD) ) },
    ( GRADLE_PROPS ) => { concat!(
        "org.gradle.jvmargs=-Xmx1G\n",
        "org.gradle.parallel=true\n",
        "minecraft_version=", ver!(MINECRAFT), "\n",
        "yarn_mappings=", ver!(LOOM_BUILD), "\n",
        "loader_version=0.16.9\n",
        "mod_version=0.1.0\n",
        "maven_group=net.totobirdcreations.voxidianprotocoldatagen\n",
        "archives_base_name=voxidian-protocol-datagen\n"
    ) }
}


fn run_datagen() {
    let dir = env::current_dir().unwrap().join("voxidian-protocol-datagen");

    { write!(fs::File::create(dir.join("gradle.properties")).unwrap(), ver!(GRADLE_PROPS)).unwrap(); }

    #[cfg(target_family = "unix")]
    let (cmd, args) = ("sh", [dir.join("gradlew.sh").into_os_string()]);

    #[cfg(target_family = "windows")]
    let (cmd, args) = ("cmd", ["/C".into(), dir.join("gradlew.bat").into_os_string()]);

    #[cfg(target_family = "wasm")]
    compile_error!("Compiling to WASM is unsupported");

    let args : Vec<OsString> = args.to_vec();

    // Refresh dependencies.
    Command::new(cmd).args({ let mut args = args.clone(); args.extend_from_slice(
        &["--refresh-dependencies".into()]
    ); args }).current_dir("voxidian-protocol-datagen").status().unwrap().exit_ok().unwrap();

    // Remove mappings cache.
    fs::remove_dir_all(dir.join("remappedSrc")).unwrap();

    // Set up mappings.
    Command::new(cmd).args({ let mut args = args.clone(); args.extend_from_slice(
        &["migrateMappings".into(), concat!("--mappings=", ver!(MAPPINGS)).into()]
    ); args }).current_dir("voxidian-protocol-datagen").status().unwrap().exit_ok().unwrap();

    // Run server/mod and datagen.
    Command::new(cmd).args({ let mut args = args.clone(); args.extend_from_slice(
        &["runServer".into()]
    ); args }).current_dir("voxidian-protocol-datagen").status().unwrap().exit_ok().unwrap();

}


fn main() {
    let dir = env::current_dir().unwrap().join("generated");
    if ([
        "packets.json"
    ].iter().any(|fname| ! dir.join(fname).is_file())) {
        run_datagen();
    }

    println!("cargo::rerun-if-changed=generated");
}

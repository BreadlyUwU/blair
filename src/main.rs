mod config;
use colored::Colorize;

use config::RunningMode;
use config::RunningMode::Production as ProdMode;
use config::RunningMode::Debug as DebugMode;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    unsafe { std::env::set_var("RUST_LOG", "debug"); }

    #[cfg(debug_assertions)]
    static RUNNING_MODE: RunningMode = DebugMode;
    #[cfg(not(debug_assertions))]
    static RUNNING_MODE: RunningMode = ProdMode;

    println!("===============================================");
    println!("=    X0:                                      =");
    println!("=   'XXX0:                            dkOk.   =");
    println!("=   'NXXXKO.                       .ONKOxdd   =");
    println!("=    XNNXXX0d                    .0NX00xddd.  =");
    println!("=    dNNNXX00xodoll::;;,.     .c0XXKK00kool.  =");
    println!("=    .XXXXXK0OkxkxxxxloodxkkkOOkkk0OOxdxdll   =");
    println!("=     kKXX0Kkxkxdoodko:;clcloxddloxxxdolllo   =");
    println!("=     kKKKO0kokdldx0koclc::clllcllocc:;;;:'   =");
    println!("=    ;K000OKOxxlxO00xc,,'',:cll:;,;;:;,;cc    =");
    println!("=   c000kOOKOOOOK000Ol,,;:::::clllll;..:ll'   =");
    println!("=  okd:..l00KKKK0KK0Oxxxlccccclcc::;.';ool:   =");
    println!("= .Odc..  c0K00KOk00kdooddoolcccc;:clloolo;   =");
    println!("= .Oodc;,;xOkkkO0kxddloddoc'.;cccclllooloo    =");
    println!("= .K000OO0kxxxkkkdxkOOOOxoc:,'':c:c:loool.    =");
    println!("=  ONXKK0OkddxxkOOO0000OOkxxxxoc;:::cclol     =");
    println!("=  .NNKKKkdoooxOOO0O00000OOOO0Odl;,;:ccl:.    =");
    println!("=    lXXK0xxOOOOkkxkkO000000O0OOkddllddl:;    =");
    println!("=     'NKK0kk0K0OkkkO00Oxk00OO0O0OOOkkkxc:;   =");
    println!("=       KWNK0OOKK00O000KK0000000000OOOOxl::,  =");
    println!("=      .WWWWNXKKKKKKK0KKKKKKKKKO00000OO0Oxlc. =");
    println!("=      XWWNNXNNWWWNXKXKKKK000000O0000K00Oocc, =");
    println!("=     ;MWWNNXXXXXXXXKO0O0O00000K000O00000d:;, =");
    println!("=     kWWNNXXXKKKKKKKKKK0OOO0O000000KK00Oklc. =");
    println!("=     KWWWNNXXKKKKKK00K00O00OO000OOO0000Okdl  =");
    println!("=     XWNNNNXKKXKKKKKKKK0000O0KK00KKO0KOOkoc  =");
    println!("===============================================");
    println!("=                 B - L A I R                 =");
    println!("=   The entry point to the Breadcat's Lair    =");
    println!("===============================================");
    println!(
        "- Version: {}\n- Mode: {}\n",
        env!("CARGO_PKG_VERSION").bold(),
        match RUNNING_MODE {
            DebugMode => "Debug (°o°)".red().bold(),
            ProdMode => "Release (^_^)".green().bold(),
        }
    );

    env_logger::init();
    println!("{}\n", "Ready! c:".blue().bold());
    blair::launch()?.await?;
    Ok(())
}

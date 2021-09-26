use structopt::StructOpt;

enum Bark {
    Whoof,
    Bork,
    Guau,
    RepeatedWhoof,
}

impl From<&str> for Bark {
    fn from(s: &str) -> Self {
        match s {
            "b" | "bork" => Self::Bork,
            "g" | "guau" => Self::Guau,
            "w" | "who-is-a-good-boy" => Self::RepeatedWhoof,
            _ => Self::Whoof,
        }
    }
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    version: bool,
    #[structopt(short, long, default_value = "whoof", parse(from_str))]
    bark: Bark,
}

const DOG_ONE: [&'static str; 5] = [
	"          __",
	" |_______/ V`-,",	
    "  }        /~~",
	" /_)^ --,r'",
	"|b      |b",
];

const VERSION: &'static str = "0.0.3";

fn main() {
    println!("{}", render(Opt::from_args()));
}

fn render(options: Opt) -> String {
    if options.version {
        format!(
            "Dog, v{}. \
             Written by Juanjo Salvador <juanjosalvador(at)netc.eu> and \
             Alejandro Domínguez <adomu(at)net-c.com>",
            VERSION,
        )
    } else {
        match options.bark {
            Bark::Whoof => render_bark("Whoof!"),
            Bark::Bork => render_bark("Bork!"),
            Bark::Guau => render_bark("Guau!"),
            Bark::RepeatedWhoof => render_bark("Whoof whoof whoof!!"),
        }
    }
}

fn render_bark(bark: &str) -> String {
    // Use .into_iter() and remove reference destructuring in .map
    // when Rust 2021 edition is released
    DOG_ONE
        .iter()
        .enumerate()
        .map(|(i, &l)| if i == 2 { format!("{} {}", l, bark) } else { l.into() })
        .collect::<Vec<_>>()
        .join("\n")
}
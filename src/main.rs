use regex::Regex;

struct Prtima {
    nirdesh: String,
    namani: Vec<String>,
}

struct Aksrm {
    prtima: Option<String>,
    basantrm: Option<String>,
    aksrm: String,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        std::process::exit(0);
    }
    let suktkrmh = &args[1];
    let suktpath = std::fs::read_to_string(suktkrmh).expect(suktkrmh);
    let gaurvm = &args[2];

    let kalah: Vec<u32> = std::fs::read_to_string(suktkrmh.to_string() + ".kalah")
        .expect(&(suktkrmh.to_string() + ".kalah"))
        .split("\n")
        .map(|s| s.parse::<u32>().unwrap() * 60 / 1000)
        .collect();

    let prtimah: Vec<Prtima> = std::fs::read_dir("prtimah/")
        .unwrap()
        .map(|p| {
            let nirdesh = p.unwrap().path();
            let nirdesnam = nirdesh.to_str().unwrap().split("/").nth(1).unwrap();
            Prtima {
                nirdesh: nirdesnam.to_string(),
                namani: nirdesnam.split("_").map(|s| s.to_string()).collect(),
            }
        })
        .collect();

    println!("{}", prtimah[0].namani[0]);

    let re =
        Regex::new(r"((\[[ a-z]+\])?) *((\{[ -Za-z]+\})?) *(([क-ह]्)*[ऄ-ह][ँ-ःऺ-ौ॑-ॗ]*)([क-ह]्[ ।])?")
            .unwrap();

    let aksrani: Vec<Aksrm> = re
        .captures_iter(&suktpath)
        .map(|aksrm| Aksrm {
            prtima: if &aksrm[1] == "" {
                None
            } else {
                Some(
                    prtimah
                        .iter()
                        .filter(|p| {
                            aksrm[1][1..aksrm[1].len() - 1]
                                .split(" ")
                                .all(|a| p.namani.iter().any(|n| n == a))
                        })
                        .next()
                        .expect(&("n lbda ".to_string() + &aksrm[1]))
                        .nirdesh
                        .clone(),
                )
            },
            basantrm: if &aksrm[3] == "" {
                None
            } else {
                Some(aksrm[3].to_string())
            },
            aksrm: aksrm[5].to_string(),
        })
        .collect();

    assert_eq!(aksrani.len(), kalah.len());

    let suktprtimah: Vec<(u32, String)> = aksrani
        .iter()
        .enumerate()
        .filter(|(_, a)| a.prtima != None)
        .map(|(i, a)| {
            (
                if i == 0 { 0 } else { kalah[i - 1] },
                a.prtima.as_ref().unwrap().to_string(),
            )
        })
        .collect();

    (0..suktprtimah.len())
        .fold(
            std::process::Command::new("melt")
                .arg("-profile")
                .arg("mlt_profile.txt"),
            |c, p| {
                c.arg(format!("prtimah/{}", suktprtimah[p].1))
                    .arg("in=0")
                    .arg(format!(
                        "out={}",
                        if p < suktprtimah.len() - 1 {
                            suktprtimah[p + 1].0
                        } else {
                            kalah[kalah.len() - 1]
                        } - suktprtimah[p].0
                    ))
            },
        )
        .arg("-filter")
        .arg("crop")
        .arg("center=1")
        .arg("-consumer")
        .arg("avformat:out.avi")
        .arg(format!("width={}", gaurvm))
        .arg(format!("height={}", gaurvm))
        .spawn();
}

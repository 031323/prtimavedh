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
        .filter(|s| !s.is_empty())
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

    for aksrm in aksrani.iter() {
        println!("{}", aksrm.aksrm);
    }
    assert_eq!(aksrani.len(), kalah.len());

    let basantrani: Vec<(u32, String)> = aksrani
        .iter()
        .enumerate()
        .filter(|(_, a)| a.basantrm != None)
        .map(|(i, a)| {
            (
                if i == 0 { 0 } else { kalah[i - 1] },
                a.basantrm.as_ref().unwrap().to_string(),
            )
        })
        .collect();

    fn fr(kalh: u32) -> String {
        let kalh = kalh * 1000 / 60;
        format!("00:{:02}:{:02},{:03}", kalh/1000/60, kalh/1000%60, kalh%1000).to_string()
    }

    let srt = (0..basantrani.len())
        .fold(("".to_string(), "".to_string()), |(srt, ba), b| {
            let ba = if ba == "" || ba.ends_with(".") {"".to_string()} else {ba + " "} + &basantrani[b].1[1..basantrani[b].1.len()-1];
            (format!("{}{}\n{} --> {}\n{}\n\n", srt, b + 1, fr(basantrani[b].0), fr(if b < basantrani.len()-1 {basantrani[b+1].0} else {kalah[kalah.len()-1]}), ba), ba)
        }).0;

    std::fs::write(format!("{}.srt", suktkrmh), srt);

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
                        } - suktprtimah[p].0 - 1
                    ))
            },
        )
        .arg("-filter")
        .arg("crop")
        .arg("center=1")
        .arg("-track")
        .arg(format!("{}.mp3", suktkrmh))
        .arg("in=0")
        .arg(format!("out={}", kalah[kalah.len() - 1] - 1))
        .arg("-consumer")
        .arg(format!("avformat:{}.mp4", suktkrmh))
        .arg("acodec=libmp3lame")
        .arg("vcodec=libx264")
        .arg(format!("width={}", gaurvm))
        .arg(format!("height={}", gaurvm))
        .spawn();
}

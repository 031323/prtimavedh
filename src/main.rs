struct Suktm {
	
}

// melt -profile mlt_profile.txt prtimah/tvm.jpg in=0 out=20 -attach watermark:+अ॒ग्निः.txt producer.align=centre composite.valign=middle composite.halign=center prtimah/tvm.jpg in=0 out=5 -filter crop center=1 -consumer avformat:out.avi width=50 height=100

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        std::process::exit(0);
    }
    let suktkrmh = &args[1];
    let suktpath = std::fs::read_to_string(suktkrmh).expect(suktkrmh);
    let suktaksrkalah = std::fs::read_to_string(suktkrmh.to_string() + ".kalah").expect(&(suktkrmh.to_string() + ".kalah"));
    println!("{}", suktaksrkalah);
}

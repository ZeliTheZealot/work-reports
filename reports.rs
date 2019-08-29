
struct Report {
    done: String,
    to_do: String,
}

struct HourReport {
    report: Report,
    date: u32,
    time: u32,
}

impl Report {
    fn new(done: String, to_do: String) -> Self {
        Report{done, to_do}
    }
    fn done(&self) -> u32 {
        self.done()
    }
    fn to_do(&self) -> u32 {
        self.to_do()
    }
}

impl HourReport {
    fn new(done: String, to_do: String, date: u32, time: u32) -> Self {
        let report = Report{done, to_do};
        HourReport{report, date, time}
    }
    fn done(&self) -> u32 {
        self.report.done()
    }
    fn to_do(&self) -> u32 {
        self.report.to_do()
    }
    fn date(&self) -> u32 {
        self.date()
    }
    fn time(&self) -> u32 {
        self.time()
    }
}

fn main() {
    let done = “multisig/schnorr.rs tests and benches; code at https://github.com/ZeliTheZealot/digital-signatures”;
    let to_do = “documentation of multisig/schnorr.rs and readme of the digital-signatures workspace”;
    let my_hour_report = HourReport::new(done, to_do, 20190829, 1200);
    println!(“This is zeliwang’s hourly report at {}hrs on the day {}.”, my_hour_report.time(), my_hour_report.date());
    println!(“I have done {}.”, my_hour_report.done());
    println!(“I will do these next: {}.”, my_hour_report.to_do());
}

//written on iPhone X
//debugged and compiled on https://tio.run/#rust
//got interesting error: left/right Unicode quotation marks are not quotation marks
//How to fix: Settings-General-Keyboard-Smart Punctuation-Off























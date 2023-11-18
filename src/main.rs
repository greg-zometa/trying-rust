enum Title {
    GeneralContractor,
    SubContractor,
    Owner
}

struct ConstructionWorker {
    title: Title,
    wage: i64
}

fn get_construction_worker(construction_worker: ConstructionWorker) {
    let pre_text = "The construction worker's title is:";
    match construction_worker.title {
        Title::GeneralContractor => println!("{pre_text} General Contractor"),
        Title::SubContractor => println!("{pre_text} Sub-Contractor"),
        Title::Owner => println!("{pre_text} Owner")
    }
    println!("They make ${:?}\n", construction_worker.wage);
}

fn main() {
    let gc = ConstructionWorker {
        title: Title::GeneralContractor,
        wage: 180000
    };
    get_construction_worker(gc);

    let sc = ConstructionWorker {
        title: Title::SubContractor,
        wage: 150000
    };
    get_construction_worker(sc);

    let owner = ConstructionWorker {
        title: Title::Owner,
        wage: 120000
    };
    get_construction_worker(owner);
}
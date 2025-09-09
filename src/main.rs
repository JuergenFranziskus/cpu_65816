use cpu_65816::{
    Bus, Cpu65816,
    core::{Core, P},
};
use serde::Deserialize;

const RAM_SIZE: usize = 16 * 1024 * 1024;

fn main() {
    let mut ram = vec![0; RAM_SIZE];
    for i in 0..=255u8 {
        let e_file = format!("65816/v1/{i:0>2x}.e.json");
        let n_file = format!("65816/v1/{i:0>2x}.n.json");

        run_test_file(i, true, &e_file, &mut ram);
        run_test_file(i, false, &n_file, &mut ram);
    }
}

fn run_test_file(i: u8, e: bool, file: &str, ram: &mut [u8]) {
    let src = std::fs::read_to_string(file).unwrap();
    let tests: Vec<Test> = serde_json::de::from_str(&src).unwrap();
    for test in tests {
        run_test(test, i, e, ram);
    }
}
fn run_test(test: Test, i: u8, e: bool, ram: &mut [u8]) {
    println!("Running test \"{}\"", test.name);

    let mut cpu = prepare_cpu(&test.start);
    let mut bus = Bus::new();
    prepare_ram(&test.start, ram);

    for &(should_addr, should_data, ref cycle) in &test.cycles {
        let should_addr = should_addr.unwrap_or(0);
        let should_data = should_data.unwrap_or(0);
        cpu.cycle(&mut bus);
        let addr = bus.linear_address() as usize;
        if bus.rw() {
            bus.data = ram[addr];
        } else {
            ram[addr] = bus.data;
        }

        compare_cycle(bus, should_addr, should_data, cycle);
    }

    compare_cpu(&test.end, cpu.core());
    compare_ram(&test.end, ram);
}
fn prepare_cpu(start: &State) -> Cpu65816 {
    let core = Core {
        a: start.a,
        d: start.d,
        dbr: start.dbr,
        e: start.e != 0,
        p: P(start.p),
        pbr: start.pbr,
        pc: start.pc,
        s: start.s,
        x: start.x,
        y: start.y,
    };

    Cpu65816::new(core)
}
fn prepare_ram(start: &State, ram: &mut [u8]) {
    for &(addr, data) in &start.ram {
        let addr = addr as usize;
        ram[addr] = data;
    }
}
fn compare_cycle(bus: Bus, should_addr: u32, should_data: u8, cycle: &str) {
    let should_bank = (should_addr >> 16) as u8;
    let should_addr = should_addr as u16;
    let mut cycle = cycle.chars();
    let should_vda = cycle.next().unwrap() == 'd';
    let should_vpa = cycle.next().unwrap() == 'p';
    let _should_vp = cycle.next().unwrap() == 'v';
    let should_rw = cycle.next().unwrap() == 'r';
    let _should_e = cycle.next().unwrap() == 'e';
    let _should_m = cycle.next().unwrap() == 'm';
    let _should_x = cycle.next().unwrap() == 'x';
    let _should_ml = cycle.next().unwrap() == 'l';

    let mut is_err = false;

    if should_bank != bus.bank {
        is_err = true;
        println!(
            "Does not match Bank,\tshould {should_bank:0>2x}, is {:0>2x}",
            bus.bank
        );
    }
    if should_addr != bus.addr {
        is_err = true;
        println!(
            "Does not match Addr,\tshould {should_addr:0>4x}, is {:0>4x}",
            bus.addr
        );
    }
    if should_data != bus.data {
        is_err = true;
        println!(
            "Does not match Data,\tshould {should_data:0>2x}, is {:0>2x}",
            bus.data
        );
    }
    if should_vda != bus.vda() {
        is_err = true;
        println!("Does not match VDA,\tshould {should_vda}, is {}", bus.vda());
    }
    if should_vpa != bus.vpa() {
        is_err = true;
        println!("Does not match VPA,\tshould {should_vpa}, is {}", bus.vpa());
    }
    if should_rw != bus.rw() {
        is_err = true;
        println!("Does not match RW,\tshould {should_rw}, is {}", bus.rw());
    }

    if is_err {
        panic!("bus activity for cycle does not match");
    }
}
fn compare_cpu(end: &State, core: Core) {
    let should_e = end.e != 0;
    let should_p = P(end.p);

    let mut is_err = false;

    if end.a != core.a {
        is_err = true;
        println!(
            "Does not match register A,\tshould {:0>4x}, is {:0>4x}",
            end.a, core.a
        );
    }
    if end.d != core.d {
        is_err = true;
        println!(
            "Does not match register D,\tshould {:0>4x}, is {:0>4x}",
            end.d, core.d
        );
    }
    if end.dbr != core.dbr {
        is_err = true;
        println!(
            "Does not match register DBR,\tshould {:0>2x}, is {:0>2x}",
            end.dbr, core.dbr
        );
    }
    if should_e != core.e {
        is_err = true;
        println!(
            "Does not match register E,\tshould {}, is {}",
            should_e, core.e
        );
    }
    if should_p != core.p {
        is_err = true;
        println!(
            "Does not match register P,\tshould {}, is {}",
            should_p, core.p
        );
    }
    if end.pbr != core.pbr {
        is_err = true;
        println!(
            "Does not match register PBR,\tshould {:0>2x}, is {:0>2x}",
            end.pbr, core.pbr
        );
    }
    if end.pc != core.pc {
        is_err = true;
        println!(
            "Does not match register PC,\tshould {:0>4x}, is {:0>4x}",
            end.pc, core.pc
        );
    }
    if end.s != core.s {
        is_err = true;
        println!(
            "Does not match register S,\tshould {:0>4x}, is {:0>4x}",
            end.s, core.s
        );
    }
    if end.x != core.x {
        is_err = true;
        println!(
            "Does not match register X,\tshould {:0>4x}, is {:0>4x}",
            end.x, core.x
        );
    }
    if end.y != core.y {
        is_err = true;
        println!(
            "Does not match register Y,\tshould {:0>4x}, is {:0>4x}",
            end.y, core.y
        );
    }

    if is_err {
        panic!("final CPU state does not match");
    }
}
fn compare_ram(end: &State, ram: &[u8]) {
    let mut is_err = false;

    for &(should_addr, should_data) in &end.ram {
        let is_data = ram[should_addr as usize];
        let should_bank = (should_addr >> 16) as u8;
        let should_addr = should_addr as u16;

        if should_data != is_data {
            is_err = true;
            println!(
                "Does not match RAM for address {should_bank:0>2x}:{should_addr:0>4x}, should {should_data:0>2x}, is {is_data:0>2x}"
            );
        }
    }

    if is_err {
        panic!("final RAM state does not match");
    }
}

#[derive(Deserialize)]
struct Test {
    name: String,
    #[serde(alias = "initial")]
    start: State,
    #[serde(alias = "final")]
    end: State,
    cycles: Vec<(Option<u32>, Option<u8>, String)>,
}

#[derive(Deserialize)]
struct State {
    a: u16,
    d: u16,
    dbr: u8,
    e: u8,
    p: u8,
    pbr: u8,
    pc: u16,
    s: u16,
    x: u16,
    y: u16,

    ram: Vec<(u32, u8)>,
}

type Registers {
    AF : u16, // A 15..7 F 7..0
    BC : u16, // B 15..7 C 7..0
    DE : u16, // D 15..7 E 7..0
    HL : u16  // H 15..7 L 7..0
}

type Port;

type CPU {
    Rs    : Registers
    PC    : u16,
    SP    : u16,
    Flags : u8, 
    Bus   : &Port
    Cycle : i32
} 

impl CPU {
    fn new() -> CPU {
        CPU {
            Rs : Registers {
                AF : 0,
                BC : 0,
                DE : 0,
                HL : 0
            },
            PC : 0x0100,
            SP : 0xFFFE,
            Flags : 0,
            Cycle : 0
        }
    }

    fn Reset(&self) -> unit {

    }
    fn Tick(&self) -> unit {

    }
    fn Read(&self, u8 addr) -> unit {

    }
    fn Write(&self, u8 addr, u8 data) -> unit {

    }
    fn ConnectBus(&self, &Port bus) -> unit {
        self.Bus = bus;
        self.Reset();
    }

}
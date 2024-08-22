pub mod diseno {
    use serde::Deserialize;
    use std::cmp::Ordering;
    #[derive(Debug, Deserialize)]
    pub enum Dia {
        Lun,
        Mar,
        Mier,
        Jue,
        Vie,
        Sab,
        Dom,
    }

    #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Deserialize)]
    pub enum H {
        H0,
        H1,
        H2,
        H3,
        H4,
        H5,
        H6,
        H7,
        H8,
        H9,
        H10,
        H11,
        H12,
        H13,
        H14,
        H15,
        H16,
        H17,
        H18,
        H19,
        H20,
        H21,
        H22,
        H23,
    }

    #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Deserialize)]
    pub enum M {
        M1,
        M2,
    }

    #[derive(Debug, Eq, PartialEq, PartialOrd, Deserialize)]
    pub struct Horario {
        pub hora: H,
        pub minuto: M,
    }
    impl Ord for Horario {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.hora.cmp(&other.hora) {
                Ordering::Equal => self.minuto.cmp(&other.minuto),
                x => x,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Clase {
        dia: Dia,
        inicio: Horario,
        ending: Horario,
    }

    #[derive(Deserialize, Debug)]
    pub struct Materia {
        prof: String,
        name: String,
        id: String,
        clases: Vec<Clase>,
    }
}
pub mod funciones {
    use super::diseno::{Clase, Dia, Materia, H, M};
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    fn read_json(path: &str) -> Result<Vec<Materia>, Box<dyn Error>> {
        let file = File::open(path);
    }
}

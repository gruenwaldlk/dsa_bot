use serde::Deserialize;
use serde::Serialize;
use std::fmt::*;

pub(crate) const TALENT_ID_ALCHIMIE: &str = "ALCHIMIE";
pub(crate) const TALENT_ID_BEKEHREN_UEBERZEUGEN: &str = "BEKEHREN_UEBERZEUGEN";
pub(crate) const TALENT_ID_BETOEREN: &str = "BETOEREN";
pub(crate) const TALENT_ID_BOOTE: &str = "BOOTE";
pub(crate) const TALENT_ID_BRETT_GLUECKSSPIEL: &str = "BRETT_GLUECKSSPIEL";
pub(crate) const TALENT_ID_EINSCHUECHTERN: &str = "EINSCHUECHTERN";
pub(crate) const TALENT_ID_ETIKETTE: &str = "ETIKETTE";
pub(crate) const TALENT_ID_FAEHRTENSUCHEN: &str = "FAEHRTENSUCHEN";
pub(crate) const TALENT_ID_FAHRZEUGE: &str = "FAHRZEUGE";
pub(crate) const TALENT_ID_FESSELN: &str = "FESSELN";
pub(crate) const TALENT_ID_FISCHEN_ANGELN: &str = "FISCHEN_ANGELN";
pub(crate) const TALENT_ID_FLIEGEN: &str = "FLIEGEN";
pub(crate) const TALENT_ID_GASSENWISSEN: &str = "GASSENWISSEN";
pub(crate) const TALENT_ID_GAUKELEI: &str = "GAUKELEI";
pub(crate) const TALENT_ID_GEOGRAPHIE: &str = "GEOGRAPHIE";
pub(crate) const TALENT_ID_GESCHICHTSWISSEN: &str = "GESCHICHTSWISSEN";
pub(crate) const TALENT_ID_GOETTER_KULTE: &str = "GOETTER_KULTE";
pub(crate) const TALENT_ID_HANDEL: &str = "HANDEL";
pub(crate) const TALENT_ID_HEILKUNDE_GIFT: &str = "HEILKUNDE_GIFT";
pub(crate) const TALENT_ID_HEILKUNDE_KRANKHEITEN: &str = "HEILKUNDE_KRANKHEITEN";
pub(crate) const TALENT_ID_HEILKUNDE_SEELE: &str = "HEILKUNDE_SEELE";
pub(crate) const TALENT_ID_HEILKUNDE_WUNDEN: &str = "HEILKUNDE_WUNDEN";
pub(crate) const TALENT_ID_HOLZBEARBEITUNG: &str = "HOLZBEARBEITUNG";
pub(crate) const TALENT_ID_KLETTERN: &str = "KLETTERN";
pub(crate) const TALENT_ID_KOERPERBEHERRSCHUNG: &str = "KOERPERBEHERRSCHUNG";
pub(crate) const TALENT_ID_KRAFTAKT: &str = "KRAFTAKT";
pub(crate) const TALENT_ID_KRIEGSKUNST: &str = "KRIEGSKUNST";
pub(crate) const TALENT_ID_LEBENSMITTELBEARBEITUNG: &str = "LEBENSMITTELBEARBEITUNG";
pub(crate) const TALENT_ID_LEDERBEARBEITUNG: &str = "LEDERBEARBEITUNG";
pub(crate) const TALENT_ID_MAGIEKUNDE: &str = "MAGIEKUNDE";
pub(crate) const TALENT_ID_MALEN_ZEICHNEN: &str = "MALEN_ZEICHNEN";
pub(crate) const TALENT_ID_MECHANIK: &str = "MECHANIK";
pub(crate) const TALENT_ID_MENSCHENKENNTNIS: &str = "MENSCHENKENNTNIS";
pub(crate) const TALENT_ID_METALLBEARBEITUNG: &str = "METALLBEARBEITUNG";
pub(crate) const TALENT_ID_MUSIZIEREN: &str = "MUSIZIEREN";
pub(crate) const TALENT_ID_ORIENTIERUNG: &str = "ORIENTIERUNG";
pub(crate) const TALENT_ID_PFLANZENKUNDE: &str = "PFLANZENKUNDE";
pub(crate) const TALENT_ID_RECHNEN: &str = "RECHNEN";
pub(crate) const TALENT_ID_RECHTSKUNDE: &str = "RECHTSKUNDE";
pub(crate) const TALENT_ID_REITEN: &str = "REITEN";
pub(crate) const TALENT_ID_SAGEN_LEGENDEN: &str = "SAGEN_LEGENDEN";
pub(crate) const TALENT_ID_SCHLOESSERKNACKEN: &str = "SCHLOESSERKNACKEN";
pub(crate) const TALENT_ID_SCHWIMMEN: &str = "SCHWIMMEN";
pub(crate) const TALENT_ID_SELBSTBEHERRSCHUNG: &str = "SELBSTBEHERRSCHUNG";
pub(crate) const TALENT_ID_SINGEN: &str = "SINGEN";
pub(crate) const TALENT_ID_SINNESSCHAERFE: &str = "SINNESSCHAERFE";
pub(crate) const TALENT_ID_SPHAERENKUNDE: &str = "SPHAERENKUNDE";
pub(crate) const TALENT_ID_STEINBEARBEITUNG: &str = "STEINBEARBEITUNG";
pub(crate) const TALENT_ID_STERNKUNDE: &str = "STERNKUNDE";
pub(crate) const TALENT_ID_STOFFBEARBEITUNG: &str = "STOFFBEARBEITUNG";
pub(crate) const TALENT_ID_TANZEN: &str = "TANZEN";
pub(crate) const TALENT_ID_TASCHENDIEBSTAHL: &str = "TASCHENDIEBSTAHL";
pub(crate) const TALENT_ID_TIERKUNDE: &str = "TIERKUNDE";
pub(crate) const TALENT_ID_UEBERREDEN: &str = "UEBERREDEN";
pub(crate) const TALENT_ID_VERBERGEN: &str = "VERBERGEN";
pub(crate) const TALENT_ID_VERKLEIDEN: &str = "VERKLEIDEN";
pub(crate) const TALENT_ID_WILDNISLEBEN: &str = "WILDNISLEBEN";
pub(crate) const TALENT_ID_WILLENSKRAFT: &str = "WILLENSKRAFT";
pub(crate) const TALENT_ID_ZECHEN: &str = "ZECHEN";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Talent {
    pub(self) name: String,
    pub(self) attribute_1_id: String,
    pub(self) attribute_2_id: String,
    pub(self) attribute_3_id: String,
    pub(self) value: u8,
}

impl Talent {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn attribute_1_id(&self) -> &str {
        &self.attribute_1_id
    }
    pub(crate) fn attribute_2_id(&self) -> &str {
        &self.attribute_2_id
    }
    pub(crate) fn attribute_3_id(&self) -> &str {
        &self.attribute_3_id
    }
    pub(crate) fn value(&self) -> u8 {
        self.value
    }

    pub(crate) fn default_talent_alchimie() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("FF"),
            name: String::from("Alchimie"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_bekehren_ueberzeugen() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("CH"),
            name: String::from("Bekehren & Überzeugen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_betoeren() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("CH"),
            attribute_3_id: String::from("CH"),
            name: String::from("Betören"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_boote() -> Self {
        Self {
            attribute_1_id: String::from("FF"),
            attribute_2_id: String::from("GE"),
            attribute_3_id: String::from("KK"),
            name: String::from("Boote"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_brett_gluecksspiel() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Brett- & Glücksspiel"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_einschuechtern() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("CH"),
            name: String::from("Einschüchtern"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_etikette() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("CH"),
            name: String::from("Etikette"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_faehrtensuchen() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("GE"),
            name: String::from("Fährtensuchen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_fahrzeuge() -> Self {
        Self {
            attribute_1_id: String::from("CH"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("KO"),
            name: String::from("Fahrzeuge"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_fesseln() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("KK"),
            name: String::from("Fesseln"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_fischen_angeln() -> Self {
        Self {
            attribute_1_id: String::from("FF"),
            attribute_2_id: String::from("GE"),
            attribute_3_id: String::from("KO"),
            name: String::from("Fischen & Angeln"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_fliegen() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("GE"),
            name: String::from("Fliegen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_gassenwissen() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("CH"),
            name: String::from("Gassenwissen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_gaukelei() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("CH"),
            attribute_3_id: String::from("FF"),
            name: String::from("Gaukelei"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_geographie() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Geographie"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_geschichtswissen() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Geschichtswissen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_goetter_kulte() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Götter & Kulte"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_handel() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("CH"),
            name: String::from("Handel"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_heilkunde_gift() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Heilkunde: Gift"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_heilkunde_krankheiten() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("KO"),
            name: String::from("Heilkunde: Krankheiten"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_heilkunde_seele() -> Self {
        Self {
            attribute_1_id: String::from("IN"),
            attribute_2_id: String::from("CH"),
            attribute_3_id: String::from("KO"),
            name: String::from("Heilkunde: Seele"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_heilkunde_wunden() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("FF"),
            name: String::from("Heilkunde: Wunden"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_holzbearbeitung() -> Self {
        Self {
            attribute_1_id: String::from("FF"),
            attribute_2_id: String::from("GE"),
            attribute_3_id: String::from("KK"),
            name: String::from("Holzbearbeitung"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_klettern() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("GE"),
            attribute_3_id: String::from("KK"),
            name: String::from("Klettern"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_koerperbeherrschung() -> Self {
        Self {
            attribute_1_id: String::from("GE"),
            attribute_2_id: String::from("GE"),
            attribute_3_id: String::from("KO"),
            name: String::from("Körperbeherrschung"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_kraftakt() -> Self {
        Self {
            attribute_1_id: String::from("KO"),
            attribute_2_id: String::from("KK"),
            attribute_3_id: String::from("KK"),
            name: String::from("Kraftakt"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_kriegskunst() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Kriegskunst"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_lebensmittelbearbeitung() -> Self {
        Self {
            attribute_1_id: String::from("IN"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("FF"),
            name: String::from("Lebensmittelbearbeitung"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_lederbearbeitung() -> Self {
        Self {
            attribute_1_id: String::from("FF"),
            attribute_2_id: String::from("GE"),
            attribute_3_id: String::from("KO"),
            name: String::from("Lederbearbeitung"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_magiekunde() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Magiekunde"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_malen_zeichnen() -> Self {
        Self {
            attribute_1_id: String::from("IN"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("FF"),
            name: String::from("Malen & Zeichnen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_mechanik() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("FF"),
            name: String::from("Mechanik"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_menschenkenntnis() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("CH"),
            name: String::from("Menschenkenntnis"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_metallbearbeitung() -> Self {
        Self {
            attribute_1_id: String::from("FF"),
            attribute_2_id: String::from("KO"),
            attribute_3_id: String::from("KK"),
            name: String::from("Metallbearbeitung"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_musizieren() -> Self {
        Self {
            attribute_1_id: String::from("CH"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("KO"),
            name: String::from("Musizieren"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_orientierung() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("IN"),
            name: String::from("Orientierung"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_pflanzenkunde() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("KO"),
            name: String::from("Pflanzenkunde"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_rechnen() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Rechnen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_rechtskunde() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Rechtskunde"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_reiten() -> Self {
        Self {
            attribute_1_id: String::from("CH"),
            attribute_2_id: String::from("GE"),
            attribute_3_id: String::from("KK"),
            name: String::from("Reiten"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_sagen_legenden() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Sagen & Legenden"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_schloesserknacken() -> Self {
        Self {
            attribute_1_id: String::from("IN"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("FF"),
            name: String::from("Schlösserknacken"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_schwimmen() -> Self {
        Self {
            attribute_1_id: String::from("GE"),
            attribute_2_id: String::from("KO"),
            attribute_3_id: String::from("KK"),
            name: String::from("Schwimmen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_selbstbeherrschung() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("MU"),
            attribute_3_id: String::from("KO"),
            name: String::from("Selbstbeherrschung"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_singen() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("CH"),
            attribute_3_id: String::from("KO"),
            name: String::from("Singen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_sinnesschaerfe() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("IN"),
            name: String::from("Sinnesschärfe"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_sphaerenkunde() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Sphärenkunde"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_steinbearbeitung() -> Self {
        Self {
            attribute_1_id: String::from("FF"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("KK"),
            name: String::from("Steinbearbeitung"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_sternkunde() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KL"),
            attribute_3_id: String::from("IN"),
            name: String::from("Sternkunde"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_stoffbearbeitung() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("FF"),
            name: String::from("Stoffbearbeitung"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_tanzen() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("CH"),
            attribute_3_id: String::from("GE"),
            name: String::from("Tanzen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_taschendiebstahl() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("FF"),
            attribute_3_id: String::from("GE"),
            name: String::from("Taschendiebstahl"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_tierkunde() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("MU"),
            attribute_3_id: String::from("CH"),
            name: String::from("Tierkunde"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_ueberreden() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("CH"),
            name: String::from("Überreden"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_verbergen() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("GE"),
            name: String::from("Verbergen"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_verkleiden() -> Self {
        Self {
            attribute_1_id: String::from("IN"),
            attribute_2_id: String::from("CH"),
            attribute_3_id: String::from("GE"),
            name: String::from("Verkleiden"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_wildnisleben() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("GE"),
            attribute_3_id: String::from("KO"),
            name: String::from("Wildnisleben"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_willenskraft() -> Self {
        Self {
            attribute_1_id: String::from("MU"),
            attribute_2_id: String::from("IN"),
            attribute_3_id: String::from("CH"),
            name: String::from("Willenskraft"),
            value: 0,
        }
    }
    pub(crate) fn default_talent_zechen() -> Self {
        Self {
            attribute_1_id: String::from("KL"),
            attribute_2_id: String::from("KO"),
            attribute_3_id: String::from("KK"),
            name: String::from("Zechen"),
            value: 0,
        }
    }
    pub(crate) fn get_default_by_id(id: &str) -> Option<Self> {
        let idu = id.to_uppercase();
        match idu.as_str() {
            TALENT_ID_ALCHIMIE => Some(Self::default_talent_alchimie()),
            TALENT_ID_BEKEHREN_UEBERZEUGEN => Some(Self::default_talent_bekehren_ueberzeugen()),
            TALENT_ID_BETOEREN => Some(Self::default_talent_betoeren()),
            TALENT_ID_BOOTE => Some(Self::default_talent_boote()),
            TALENT_ID_BRETT_GLUECKSSPIEL => Some(Self::default_talent_brett_gluecksspiel()),
            TALENT_ID_EINSCHUECHTERN => Some(Self::default_talent_einschuechtern()),
            TALENT_ID_ETIKETTE => Some(Self::default_talent_etikette()),
            TALENT_ID_FAEHRTENSUCHEN => Some(Self::default_talent_faehrtensuchen()),
            TALENT_ID_FAHRZEUGE => Some(Self::default_talent_fahrzeuge()),
            TALENT_ID_FESSELN => Some(Self::default_talent_fesseln()),
            TALENT_ID_FISCHEN_ANGELN => Some(Self::default_talent_fischen_angeln()),
            TALENT_ID_FLIEGEN => Some(Self::default_talent_fliegen()),
            TALENT_ID_GASSENWISSEN => Some(Self::default_talent_gassenwissen()),
            TALENT_ID_GAUKELEI => Some(Self::default_talent_gaukelei()),
            TALENT_ID_GEOGRAPHIE => Some(Self::default_talent_geographie()),
            TALENT_ID_GESCHICHTSWISSEN => Some(Self::default_talent_geschichtswissen()),
            TALENT_ID_GOETTER_KULTE => Some(Self::default_talent_goetter_kulte()),
            TALENT_ID_HANDEL => Some(Self::default_talent_handel()),
            TALENT_ID_HEILKUNDE_GIFT => Some(Self::default_talent_heilkunde_gift()),
            TALENT_ID_HEILKUNDE_KRANKHEITEN => Some(Self::default_talent_heilkunde_krankheiten()),
            TALENT_ID_HEILKUNDE_SEELE => Some(Self::default_talent_heilkunde_seele()),
            TALENT_ID_HEILKUNDE_WUNDEN => Some(Self::default_talent_heilkunde_wunden()),
            TALENT_ID_HOLZBEARBEITUNG => Some(Self::default_talent_holzbearbeitung()),
            TALENT_ID_KLETTERN => Some(Self::default_talent_klettern()),
            TALENT_ID_KOERPERBEHERRSCHUNG => Some(Self::default_talent_koerperbeherrschung()),
            TALENT_ID_KRAFTAKT => Some(Self::default_talent_kraftakt()),
            TALENT_ID_KRIEGSKUNST => Some(Self::default_talent_kriegskunst()),
            TALENT_ID_LEBENSMITTELBEARBEITUNG => {
                Some(Self::default_talent_lebensmittelbearbeitung())
            }
            TALENT_ID_LEDERBEARBEITUNG => Some(Self::default_talent_lederbearbeitung()),
            TALENT_ID_MAGIEKUNDE => Some(Self::default_talent_magiekunde()),
            TALENT_ID_MALEN_ZEICHNEN => Some(Self::default_talent_malen_zeichnen()),
            TALENT_ID_MECHANIK => Some(Self::default_talent_mechanik()),
            TALENT_ID_MENSCHENKENNTNIS => Some(Self::default_talent_menschenkenntnis()),
            TALENT_ID_METALLBEARBEITUNG => Some(Self::default_talent_metallbearbeitung()),
            TALENT_ID_MUSIZIEREN => Some(Self::default_talent_musizieren()),
            TALENT_ID_ORIENTIERUNG => Some(Self::default_talent_orientierung()),
            TALENT_ID_PFLANZENKUNDE => Some(Self::default_talent_pflanzenkunde()),
            TALENT_ID_RECHNEN => Some(Self::default_talent_rechnen()),
            TALENT_ID_RECHTSKUNDE => Some(Self::default_talent_rechtskunde()),
            TALENT_ID_REITEN => Some(Self::default_talent_reiten()),
            TALENT_ID_SAGEN_LEGENDEN => Some(Self::default_talent_sagen_legenden()),
            TALENT_ID_SCHLOESSERKNACKEN => Some(Self::default_talent_schloesserknacken()),
            TALENT_ID_SCHWIMMEN => Some(Self::default_talent_schwimmen()),
            TALENT_ID_SELBSTBEHERRSCHUNG => Some(Self::default_talent_selbstbeherrschung()),
            TALENT_ID_SINGEN => Some(Self::default_talent_singen()),
            TALENT_ID_SINNESSCHAERFE => Some(Self::default_talent_sinnesschaerfe()),
            TALENT_ID_SPHAERENKUNDE => Some(Self::default_talent_sphaerenkunde()),
            TALENT_ID_STEINBEARBEITUNG => Some(Self::default_talent_steinbearbeitung()),
            TALENT_ID_STERNKUNDE => Some(Self::default_talent_sternkunde()),
            TALENT_ID_STOFFBEARBEITUNG => Some(Self::default_talent_stoffbearbeitung()),
            TALENT_ID_TANZEN => Some(Self::default_talent_tanzen()),
            TALENT_ID_TASCHENDIEBSTAHL => Some(Self::default_talent_taschendiebstahl()),
            TALENT_ID_TIERKUNDE => Some(Self::default_talent_tierkunde()),
            TALENT_ID_UEBERREDEN => Some(Self::default_talent_ueberreden()),
            TALENT_ID_VERBERGEN => Some(Self::default_talent_verbergen()),
            TALENT_ID_VERKLEIDEN => Some(Self::default_talent_verkleiden()),
            TALENT_ID_WILDNISLEBEN => Some(Self::default_talent_wildnisleben()),
            TALENT_ID_WILLENSKRAFT => Some(Self::default_talent_willenskraft()),
            TALENT_ID_ZECHEN => Some(Self::default_talent_zechen()),
            _ => None,
        }
    }
    pub(crate) fn clean_talent_id(id: &str) -> &str {
        match id {
            "Fliegen" | "fliegen" | "FLIEGEN" => TALENT_ID_FLIEGEN,
            "Gaukelei" | "gaukelei" | "GAUKELEI" => TALENT_ID_GAUKELEI,
            "Klettern" | "klettern" | "KLETTERN" => TALENT_ID_KLETTERN,
            "Koerperbeherrschung"
            | "koerperbeherrschung"
            | "KOERPERBEHERRSCHUNG"
            | "Körperbeherrschung"
            | "körperbeherrschung"
            | "KÖRPERBEHERRSCHUNG" => TALENT_ID_KOERPERBEHERRSCHUNG,
            "Kraftakt" | "kraftakt" | "KRAFTAKT" => TALENT_ID_KRAFTAKT,
            "Reiten" | "reiten" | "REITEN" => TALENT_ID_REITEN,
            "Schwimmen" | "schwimmen" | "SCHWIMMEN" => TALENT_ID_SCHWIMMEN,
            "Selbstbeherrschung" | "selbstbeherrschung" | "SELBSTBEHERRSCHUNG" => {
                TALENT_ID_SELBSTBEHERRSCHUNG
            }

            "Singen" | "singen" | "SINGEN" => TALENT_ID_SINGEN,
            "Sinnesschaerfe" | "sinnesschaerfe" | "SINNESSCHAERFE" | "Sinnesschärfe"
            | "sinnesschärfe" | "SINNESSCHÄRFE" => TALENT_ID_SINNESSCHAERFE,
            "Tanzen" | "tanzen" | "TANZEN" => TALENT_ID_TANZEN,
            "Taschendiebstahl" | "taschendiebstahl" | "TASCHENDIEBSTAHL" => {
                TALENT_ID_TASCHENDIEBSTAHL
            }

            "Verbergen" | "verbergen" | "VERBERGEN" => TALENT_ID_VERBERGEN,
            "Zechen" | "zechen" | "ZECHEN" => TALENT_ID_ZECHEN,
            "BekehrenUeberzeugen"
            | "bekehrenueberzeugen"
            | "BEKEHRENUEBERZEUGEN"
            | "Bekehren"
            | "BEKEHREN"
            | "bekehren"
            | "Ueberzeugen"
            | "ueberzeugen"
            | "UEBERZEUGEN"
            | "BekehrenÜberzeugen"
            | "bekehrenüberzeugen"
            | "BEKEHRENÜBERZEUGEN"
            | "Überzeugen"
            | "überzeugen"
            | "ÜBERZEUGEN" => TALENT_ID_BEKEHREN_UEBERZEUGEN,
            "Betoeren" | "betoeren" | "BETOEREN" | "Betören" | "betören" | "BETÖREN" => {
                TALENT_ID_BETOEREN
            }
            "Einschuechtern" | "einschuechtern" | "EINSCHUECHTERN" | "Einschüchtern"
            | "einschüchtern" | "EINSCHÜCHTERN" => TALENT_ID_EINSCHUECHTERN,
            "Etikette" | "etikette" | "ETIKETTE" => TALENT_ID_ETIKETTE,
            "Gassenwissen" | "gassenwissen" | "GASSENWISSEN" => TALENT_ID_GASSENWISSEN,
            "Menschenkenntnis" | "menschenkenntnis" | "MENSCHENKENNTNIS" => {
                TALENT_ID_MENSCHENKENNTNIS
            }
            "Ueberreden" | "ueberreden" | "UEBERREDEN" | "Überreden" | "überreden"
            | "ÜBERREDEN" => TALENT_ID_UEBERREDEN,
            "Verkleiden" | "verkleiden" | "VERKLEIDEN" => TALENT_ID_VERKLEIDEN,
            "Willenskraft" | "willenskraft" | "WILLENSKRAFT" => TALENT_ID_WILLENSKRAFT,
            "Faehrtensuchen" | "faehrtensuchen" | "FAEHRTENSUCHEN" | "Fährtensuchen"
            | "fährtensuchen" | "FÄHRTENSUCHEN" => TALENT_ID_FAEHRTENSUCHEN,
            "Fesseln" | "fesseln" | "FESSELN" => TALENT_ID_FESSELN,
            "FischenAngeln" | "fischenangeln" | "FISCHENANGELN" | "Fischen" | "fischen"
            | "FISCHEN" | "Angeln" | "angeln" | "ANGELN" => TALENT_ID_FISCHEN_ANGELN,
            "Orientierung" | "orientierung" | "ORIENTIERUNG" => TALENT_ID_ORIENTIERUNG,
            "Pflanzenkunde" | "pflanzenkunde" | "PFLANZENKUNDE" => TALENT_ID_PFLANZENKUNDE,
            "Tierkunde" | "tierkunde" | "TIERKUNDE" => TALENT_ID_TIERKUNDE,
            "Wildnisleben" | "wildnisleben" | "WILDNISLEBEN" => TALENT_ID_WILDNISLEBEN,
            "Brettspiel" | "brettspiel" | "BRETTSPIEL" | "Brettspiele" | "brettspiele"
            | "BRETTSPIELE" | "Gluecksspiel" | "gluecksspiel" | "GLUECKSSPIEL" | "Glücksspiel"
            | "glücksspiel" | "GLÜCKSSPIEL" => TALENT_ID_BRETT_GLUECKSSPIEL,
            "Geographie" | "geographie" | "GEOGRAPHIE" => TALENT_ID_GEOGRAPHIE,
            "Geschichtswissen" | "geschichtswissen" | "GESCHICHTSWISSEN" => {
                TALENT_ID_GESCHICHTSWISSEN
            }
            "goetterkulte" | "götterkulte" | "GoetterKulte" | "GötterKulte" | "GOETTERKULTE"
            | "GÖTTERKULTE" | "goetter" | "kulte" | "götter" | "Goetter" | "Kulte" | "Götter"
            | "GOETTER" | "KULTE" | "GÖTTER" => TALENT_ID_GOETTER_KULTE,
            "Kriegskunst" | "kriegskunst" | "KRIEGSKUNST" => TALENT_ID_KRIEGSKUNST,
            "Magiekunde" | "magiekunde" | "MAGIEKUNDE" => TALENT_ID_MAGIEKUNDE,
            "Mechanik" | "mechanik" | "MECHANIK" => TALENT_ID_MECHANIK,
            "Rechnen" | "rechnen" | "RECHNEN" => TALENT_ID_RECHNEN,
            "Rechtskunde" | "rechtskunde" | "RECHTSKUNDE" => TALENT_ID_RECHTSKUNDE,
            "SagenLegenden" | "sagenlegenden" | "SAGENLEGENDEN" | "Sagen" | "sagen" | "SAGEN"
            | "Legenden" | "legenden" | "LEGENDEN" => TALENT_ID_SAGEN_LEGENDEN,
            "Sphaerenkunde" | "sphaerenkunde" | "SPHAERENKUNDE" | "Sphärenkunde"
            | "sphärenkunde" | "SPHÄRENKUNDE" => TALENT_ID_SPHAERENKUNDE,
            "Sternkunde" | "sternkunde" | "STERNKUNDE" => TALENT_ID_STERNKUNDE,
            "Alchimie" | "alchimie" | "ALCHIMIE" => TALENT_ID_ALCHIMIE,
            "Boote" | "boote" | "BOOTE" => TALENT_ID_BOOTE,
            "Fahrzeuge" | "fahrzeuge" | "FAHRZEUGE" => TALENT_ID_FAHRZEUGE,
            "Handel" | "handel" | "HANDEL" => TALENT_ID_HANDEL,
            "HeilkundeGift" | "heilkundegift" | "HEILKUNDEGIFT" => TALENT_ID_HEILKUNDE_GIFT,
            "HeilkundeKrankheiten" | "heilkundekrankheiten" | "HEILKUNDEKRANKHEITEN" => {
                TALENT_ID_HEILKUNDE_KRANKHEITEN
            }
            "HeilkundeSeele" | "heilkundeseele" | "HEILKUNDESEELE" => TALENT_ID_HEILKUNDE_SEELE,
            "HeilkundeWunden" | "heilkundewunden" | "HEILKUNDEWUNDEN" => TALENT_ID_HEILKUNDE_WUNDEN,
            "Holzbearbeitung" | "holzbearbeitung" | "HOLZBEARBEITUNG" => TALENT_ID_HOLZBEARBEITUNG,
            "Lebensmittelbearbeitung" | "lebensmittelbearbeitung" | "LEBENSMITTELBEARBEITUNG" => {
                TALENT_ID_LEBENSMITTELBEARBEITUNG
            }
            "Lederbearbeitung" | "lederbearbeitung" | "LEDERBEARBEITUNG" => {
                TALENT_ID_LEDERBEARBEITUNG
            }
            "MalenZeichnen" | "malenzeichnen" | "MALENZEICHNEN" | "Malen" | "Zeichnen"
            | "malen" | "zeichnen" | "MALEN" | "ZEICHNEN" => TALENT_ID_MALEN_ZEICHNEN,
            "Metallbearbeitung" | "metallbearbeitung" | "METALLBEARBEITUNG" => {
                TALENT_ID_METALLBEARBEITUNG
            }
            "Musizieren" | "musizieren" | "MUSIZIEREN" => TALENT_ID_MUSIZIEREN,
            "Schloesserknacken" | "schloesserknacken" | "SCHLOESSERKNACKEN"
            | "Schlösserknacken" | "schlösserknacken" | "SCHLÖSSERKNACKEN" => {
                TALENT_ID_SCHLOESSERKNACKEN
            }
            "Steinbearbeitung" | "steinbearbeitung" | "STEINBEARBEITUNG" => {
                TALENT_ID_STEINBEARBEITUNG
            }
            "Stoffbearbeitung" | "stoffbearbeitung" | "STOFFBEARBEITUNG" => {
                TALENT_ID_STOFFBEARBEITUNG
            }
            _ => "ERROR",
        }
    }
}

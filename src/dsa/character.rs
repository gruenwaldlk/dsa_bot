use crate::dsa::attribute::Attribute;
use crate::dsa::talent::Talent;
use crate::dsa::talent::TalentCheckResult;
use crate::dsa::ResultType;
use crate::dsa::*;
use crate::lib::dice::Dice;
use crate::lib::*;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Character {
    pub(self) name: String,
    pub(self) owner: u64,
    pub(self) attribute_mu: Attribute,
    pub(self) attribute_kl: Attribute,
    pub(self) attribute_in: Attribute,
    pub(self) attribute_ch: Attribute,
    pub(self) attribute_ff: Attribute,
    pub(self) attribute_ge: Attribute,
    pub(self) attribute_ko: Attribute,
    pub(self) attribute_kk: Attribute,
    pub(self) talent_fliegen: Talent,
    pub(self) talent_gaukeleien: Talent,
    pub(self) talent_klettern: Talent,
    pub(self) talent_koerperbeherrschung: Talent,
    pub(self) talent_kraftakt: Talent,
    pub(self) talent_reiten: Talent,
    pub(self) talent_schwimmen: Talent,
    pub(self) talent_selbstbeherrschung: Talent,
    pub(self) talent_singen: Talent,
    pub(self) talent_sinnesschaerfe: Talent,
    pub(self) talent_tanzen: Talent,
    pub(self) talent_taschendiebstahl: Talent,
    pub(self) talent_verbergen: Talent,
    pub(self) talent_zechen: Talent,
    pub(self) talent_bekehren_ueberzeugen: Talent,
    pub(self) talent_betoeren: Talent,
    pub(self) talent_einschuechtern: Talent,
    pub(self) talent_etikette: Talent,
    pub(self) talent_gassenwissen: Talent,
    pub(self) talent_menschenkenntnis: Talent,
    pub(self) talent_ueberreden: Talent,
    pub(self) talent_verkleiden: Talent,
    pub(self) talent_wilenskraft: Talent,
    pub(self) talent_faehrtensuchen: Talent,
    pub(self) talent_fesseln: Talent,
    pub(self) talent_fischen_angeln: Talent,
    pub(self) talent_orientierung: Talent,
    pub(self) talent_pflanzenkunde: Talent,
    pub(self) talent_tierkunde: Talent,
    pub(self) talent_wildnisleben: Talent,
    pub(self) talent_brettspiel: Talent,
    pub(self) talent_geographie: Talent,
    pub(self) talent_geschichtswissen: Talent,
    pub(self) talent_goetter_kulte: Talent,
    pub(self) talent_kriegskunst: Talent,
    pub(self) talent_magiekunde: Talent,
    pub(self) talent_mechanik: Talent,
    pub(self) talent_rechnen: Talent,
    pub(self) talent_rechtskunde: Talent,
    pub(self) talent_sagen_legenden: Talent,
    pub(self) talent_sphaerenkunde: Talent,
    pub(self) talent_sternkunde: Talent,
    pub(self) talent_alchimie: Talent,
    pub(self) talent_boote: Talent,
    pub(self) talent_fahrzeuge: Talent,
    pub(self) talent_handel: Talent,
    pub(self) talent_heilkunde_gift: Talent,
    pub(self) talent_heilkunde_krankheiten: Talent,
    pub(self) talent_heilkunde_seele: Talent,
    pub(self) talent_heilkunde_wunden: Talent,
    pub(self) talent_holzbearbeitung: Talent,
    pub(self) talent_lebensmittelbearbeitung: Talent,
    pub(self) talent_lederbearbeitung: Talent,
    pub(self) talent_malen_zeichnen: Talent,
    pub(self) talent_metallarbeitung: Talent,
    pub(self) talent_musizieren: Talent,
    pub(self) talent_schloesserknacken: Talent,
    pub(self) talent_steinbearbeitung: Talent,
    pub(self) talent_stoffbearbeitung: Talent,
}

impl Character {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn owner(&self) -> u64 {
        self.owner
    }
    pub(crate) fn attribute_mu(&self) -> &Attribute {
        &self.attribute_mu
    }
    pub(crate) fn attribute_kl(&self) -> &Attribute {
        &self.attribute_kl
    }
    pub(crate) fn attribute_in(&self) -> &Attribute {
        &self.attribute_in
    }
    pub(crate) fn attribute_ch(&self) -> &Attribute {
        &self.attribute_ch
    }
    pub(crate) fn attribute_ff(&self) -> &Attribute {
        &self.attribute_ff
    }
    pub(crate) fn attribute_ge(&self) -> &Attribute {
        &self.attribute_ge
    }
    pub(crate) fn attribute_ko(&self) -> &Attribute {
        &self.attribute_ko
    }
    pub(crate) fn attribute_kk(&self) -> &Attribute {
        &self.attribute_kk
    }
    pub(crate) fn get_attribute_by_id(&self, id: &str) -> Option<&Attribute> {
        match id {
            "mu" | "MU" | "Mu" | "mU" => Some(self.attribute_mu()),
            "kl" | "KL" | "Kl" | "kL" => Some(self.attribute_kl()),
            "in" | "IN" | "In" | "iN" => Some(self.attribute_in()),
            "ch" | "CH" | "Ch" | "cH" => Some(self.attribute_ch()),
            "ff" | "FF" | "Ff" | "fF" => Some(self.attribute_ff()),
            "ge" | "GE" | "Ge" | "gE" => Some(self.attribute_ge()),
            "ko" | "KO" | "Ko" | "kO" => Some(self.attribute_ko()),
            "kk" | "KK" | "Kk" | "kK" => Some(self.attribute_kk()),
            _ => None,
        }
    }
    pub(crate) fn talent_fliegen(&self) -> &Talent {
        &self.talent_fliegen
    }
    pub(crate) fn talent_gaukeleien(&self) -> &Talent {
        &self.talent_gaukeleien
    }
    pub(crate) fn talent_klettern(&self) -> &Talent {
        &self.talent_klettern
    }
    pub(crate) fn talent_koerperbeherrschung(&self) -> &Talent {
        &self.talent_koerperbeherrschung
    }
    pub(crate) fn talent_kraftakt(&self) -> &Talent {
        &self.talent_kraftakt
    }
    pub(crate) fn talent_reiten(&self) -> &Talent {
        &self.talent_reiten
    }
    pub(crate) fn talent_schwimmen(&self) -> &Talent {
        &self.talent_schwimmen
    }
    pub(crate) fn talent_selbstbeherrschung(&self) -> &Talent {
        &self.talent_selbstbeherrschung
    }
    pub(crate) fn talent_singen(&self) -> &Talent {
        &self.talent_singen
    }
    pub(crate) fn talent_sinnesschaerfe(&self) -> &Talent {
        &self.talent_sinnesschaerfe
    }
    pub(crate) fn talent_tanzen(&self) -> &Talent {
        &self.talent_tanzen
    }
    pub(crate) fn talent_taschendiebstahl(&self) -> &Talent {
        &self.talent_taschendiebstahl
    }
    pub(crate) fn talent_verbergen(&self) -> &Talent {
        &self.talent_verbergen
    }
    pub(crate) fn talent_zechen(&self) -> &Talent {
        &self.talent_zechen
    }
    pub(crate) fn talent_bekehren_ueberzeugen(&self) -> &Talent {
        &self.talent_bekehren_ueberzeugen
    }
    pub(crate) fn talent_betoeren(&self) -> &Talent {
        &self.talent_betoeren
    }
    pub(crate) fn talent_einschuechtern(&self) -> &Talent {
        &self.talent_einschuechtern
    }
    pub(crate) fn talent_etikette(&self) -> &Talent {
        &self.talent_etikette
    }
    pub(crate) fn talent_gassenwissen(&self) -> &Talent {
        &self.talent_gassenwissen
    }
    pub(crate) fn talent_menschenkenntnis(&self) -> &Talent {
        &self.talent_menschenkenntnis
    }
    pub(crate) fn talent_ueberreden(&self) -> &Talent {
        &self.talent_ueberreden
    }
    pub(crate) fn talent_verkleiden(&self) -> &Talent {
        &self.talent_verkleiden
    }
    pub(crate) fn talent_wilenskraft(&self) -> &Talent {
        &self.talent_wilenskraft
    }
    pub(crate) fn talent_faehrtensuchen(&self) -> &Talent {
        &self.talent_faehrtensuchen
    }
    pub(crate) fn talent_fesseln(&self) -> &Talent {
        &self.talent_fesseln
    }
    pub(crate) fn talent_fischen_angeln(&self) -> &Talent {
        &self.talent_fischen_angeln
    }
    pub(crate) fn talent_orientierung(&self) -> &Talent {
        &self.talent_orientierung
    }
    pub(crate) fn talent_pflanzenkunde(&self) -> &Talent {
        &self.talent_pflanzenkunde
    }
    pub(crate) fn talent_tierkunde(&self) -> &Talent {
        &self.talent_tierkunde
    }
    pub(crate) fn talent_wildnisleben(&self) -> &Talent {
        &self.talent_wildnisleben
    }
    pub(crate) fn talent_brettspiel(&self) -> &Talent {
        &self.talent_brettspiel
    }
    pub(crate) fn talent_geographie(&self) -> &Talent {
        &self.talent_geographie
    }
    pub(crate) fn talent_geschichtswissen(&self) -> &Talent {
        &self.talent_geschichtswissen
    }
    pub(crate) fn talent_goetter_kulte(&self) -> &Talent {
        &self.talent_goetter_kulte
    }
    pub(crate) fn talent_kriegskunst(&self) -> &Talent {
        &self.talent_kriegskunst
    }
    pub(crate) fn talent_magiekunde(&self) -> &Talent {
        &self.talent_magiekunde
    }
    pub(crate) fn talent_mechanik(&self) -> &Talent {
        &self.talent_mechanik
    }
    pub(crate) fn talent_rechnen(&self) -> &Talent {
        &self.talent_rechnen
    }
    pub(crate) fn talent_rechtskunde(&self) -> &Talent {
        &self.talent_rechtskunde
    }
    pub(crate) fn talent_sagen_legenden(&self) -> &Talent {
        &self.talent_sagen_legenden
    }
    pub(crate) fn talent_sphaerenkunde(&self) -> &Talent {
        &self.talent_sphaerenkunde
    }
    pub(crate) fn talent_sternkunde(&self) -> &Talent {
        &self.talent_sternkunde
    }
    pub(crate) fn talent_alchimie(&self) -> &Talent {
        &self.talent_alchimie
    }
    pub(crate) fn talent_boote(&self) -> &Talent {
        &self.talent_boote
    }
    pub(crate) fn talent_fahrzeuge(&self) -> &Talent {
        &self.talent_fahrzeuge
    }
    pub(crate) fn talent_handel(&self) -> &Talent {
        &self.talent_handel
    }
    pub(crate) fn talent_heilkunde_gift(&self) -> &Talent {
        &self.talent_heilkunde_gift
    }
    pub(crate) fn talent_heilkunde_krankheiten(&self) -> &Talent {
        &self.talent_heilkunde_krankheiten
    }
    pub(crate) fn talent_heilkunde_seele(&self) -> &Talent {
        &self.talent_heilkunde_seele
    }
    pub(crate) fn talent_heilkunde_wunden(&self) -> &Talent {
        &self.talent_heilkunde_wunden
    }
    pub(crate) fn talent_holzbearbeitung(&self) -> &Talent {
        &self.talent_holzbearbeitung
    }
    pub(crate) fn talent_lebensmittelbearbeitung(&self) -> &Talent {
        &self.talent_lebensmittelbearbeitung
    }
    pub(crate) fn talent_lederbearbeitung(&self) -> &Talent {
        &self.talent_lederbearbeitung
    }
    pub(crate) fn talent_malen_zeichnen(&self) -> &Talent {
        &self.talent_malen_zeichnen
    }
    pub(crate) fn talent_metallarbeitung(&self) -> &Talent {
        &self.talent_metallarbeitung
    }
    pub(crate) fn talent_musizieren(&self) -> &Talent {
        &self.talent_musizieren
    }
    pub(crate) fn talent_schloesserknacken(&self) -> &Talent {
        &self.talent_schloesserknacken
    }
    pub(crate) fn talent_steinbearbeitung(&self) -> &Talent {
        &self.talent_steinbearbeitung
    }
    pub(crate) fn talent_stoffbearbeitung(&self) -> &Talent {
        &self.talent_stoffbearbeitung
    }
    pub(crate) fn get_talent_by_id(&self, id: &str) -> Option<&Talent> {
        match id {
            "Fliegen" | "fliegen" | "FLIEGEN" => Some(self.talent_fliegen()),
            "Gaukelei" | "gaukelei" | "GAUKELEI" => Some(self.talent_gaukeleien()),
            "Klettern" | "klettern" | "KLETTERN" => Some(self.talent_klettern()),
            "Koerperbeherrschung"
            | "koerperbeherrschung"
            | "KOERPERBEHERRSCHUNG"
            | "Körperbeherrschung"
            | "körperbeherrschung"
            | "KÖRPERBEHERRSCHUNG" => Some(self.talent_koerperbeherrschung()),
            "Kraftakt" | "kraftakt" | "KRAFTAKT" => Some(self.talent_kraftakt()),
            "Reiten" | "reiten" | "REITEN" => Some(self.talent_reiten()),
            "Schwimmen" | "schwimmen" | "SCHWIMMEN" => Some(self.talent_schwimmen()),
            "Selbstbeherrschung" | "selbstbeherrschung" | "SELBSTBEHERRSCHUNG" => {
                Some(self.talent_selbstbeherrschung())
            }
            "Singen" | "singen" | "SINGEN" => Some(self.talent_singen()),
            "Sinnesschaerfe" | "sinnesschaerfe" | "SINNESSCHAERFE" | "Sinnesschärfe"
            | "sinnesschärfe" | "SINNESSCHÄRFE" => Some(self.talent_sinnesschaerfe()),
            "Tanzen" | "tanzen" | "TANZEN" => Some(self.talent_tanzen()),
            "Taschendiebstahl" | "taschendiebstahl" | "TASCHENDIEBSTAHL" => {
                Some(self.talent_taschendiebstahl())
            }
            "Verbergen" | "verbergen" | "VERBERGEN" => Some(self.talent_verbergen()),
            "Zechen" | "zechen" | "ZECHEN" => Some(self.talent_zechen()),
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
            | "ÜBERZEUGEN" => Some(self.talent_bekehren_ueberzeugen()),
            "Betoeren" | "betoeren" | "BETOEREN" | "Betören" | "betören" | "BETÖREN" => {
                Some(self.talent_betoeren())
            }
            "Einschuechtern" | "einschuechtern" | "EINSCHUECHTERN" | "Einschüchtern"
            | "einschüchtern" | "EINSCHÜCHTERN" => Some(self.talent_einschuechtern()),
            "Etikette" | "etikette" | "ETIKETTE" => Some(self.talent_etikette()),
            "Gassenwissen" | "gassenwissen" | "GASSENWISSEN" => Some(self.talent_gassenwissen()),
            "Menschenkenntnis" | "menschenkenntnis" | "MENSCHENKENNTNIS" => {
                Some(self.talent_menschenkenntnis())
            }
            "Ueberreden" | "ueberreden" | "UEBERREDEN" | "Überreden" | "überreden"
            | "ÜBERREDEN" => Some(self.talent_ueberreden()),
            "Verkleiden" | "verkleiden" | "VERKLEIDEN" => Some(self.talent_verkleiden()),
            "Willenskraft" | "willenskraft" | "WILLENSKRAFT" => Some(self.talent_wilenskraft()),
            "Faehrtensuchen" | "faehrtensuchen" | "FAEHRTENSUCHEN" | "Fährtensuchen"
            | "fährtensuchen" | "FÄHRTENSUCHEN" => Some(self.talent_faehrtensuchen()),
            "Fesseln" | "fesseln" | "FESSELN" => Some(self.talent_fesseln()),
            "FischenAngeln" | "fischenangeln" | "FISCHENANGELN" | "Fischen" | "fischen"
            | "FISCHEN" | "Angeln" | "angeln" | "ANGELN" => Some(self.talent_fischen_angeln()),
            "Orientierung" | "orientierung" | "ORIENTIERUNG" => Some(self.talent_orientierung()),
            "Pflanzenkunde" | "pflanzenkunde" | "PFLANZENKUNDE" => {
                Some(self.talent_pflanzenkunde())
            }
            "Tierkunde" | "tierkunde" | "TIERKUNDE" => Some(self.talent_tierkunde()),
            "Wildnisleben" | "wildnisleben" | "WILDNISLEBEN" => Some(self.talent_wildnisleben()),
            "Brettspiel" | "brettspiel" | "BRETTSPIEL" | "Brettspiele" | "brettspiele"
            | "BRETTSPIELE" | "Gluecksspiel" | "gluecksspiel" | "GLUECKSSPIEL" | "Glücksspiel"
            | "glücksspiel" | "GLÜCKSSPIEL" => Some(self.talent_brettspiel()),
            "Geographie" | "geographie" | "GEOGRAPHIE" => Some(self.talent_geographie()),
            "Geschichtswissen" | "geschichtswissen" | "GESCHICHTSWISSEN" => {
                Some(self.talent_geschichtswissen())
            }
            "goetterkulte" | "götterkulte" | "GoetterKulte" | "GötterKulte" | "GOETTERKULTE"
            | "GÖTTERKULTE" | "goetter" | "kulte" | "götter" | "Goetter" | "Kulte" | "Götter"
            | "GOETTER" | "KULTE" | "GÖTTER" => Some(self.talent_goetter_kulte()),
            "Kriegskunst" | "kriegskunst" | "KRIEGSKUNST" => Some(self.talent_kriegskunst()),
            "Magiekunde" | "magiekunde" | "MAGIEKUNDE" => Some(self.talent_magiekunde()),
            "Mechanik" | "mechanik" | "MECHANIK" => Some(self.talent_mechanik()),
            "Rechnen" | "rechnen" | "RECHNEN" => Some(self.talent_rechnen()),
            "Rechtskunde" | "rechtskunde" | "RECHTSKUNDE" => Some(self.talent_rechtskunde()),
            "SagenLegenden" | "sagenlegenden" | "SAGENLEGENDEN" | "Sagen" | "sagen" | "SAGEN"
            | "Legenden" | "legenden" | "LEGENDEN" => Some(self.talent_sagen_legenden()),
            "Sphaerenkunde" | "sphaerenkunde" | "SPHAERENKUNDE" | "Sphärenkunde"
            | "sphärenkunde" | "SPHÄRENKUNDE" => Some(self.talent_sphaerenkunde()),
            "Sternkunde" | "sternkunde" | "STERNKUNDE" => Some(self.talent_sternkunde()),
            "Alchimie" | "alchimie" | "ALCHIMIE" => Some(self.talent_alchimie()),
            "Boote" | "boote" | "BOOTE" => Some(self.talent_boote()),
            "Fahrzeuge" | "fahrzeuge" | "FAHRZEUGE" => Some(self.talent_fahrzeuge()),
            "Handel" | "handel" | "HANDEL" => Some(self.talent_handel()),
            "HeilkundeGift" | "heilkundegift" | "HEILKUNDEGIFT" => {
                Some(self.talent_heilkunde_gift())
            }
            "HeilkundeKrankheiten" | "heilkundekrankheiten" | "HEILKUNDEKRANKHEITEN" => {
                Some(self.talent_heilkunde_krankheiten())
            }
            "HeilkundeSeele" | "heilkundeseele" | "HEILKUNDESEELE" => {
                Some(self.talent_heilkunde_seele())
            }
            "HeilkundeWunden" | "heilkundewunden" | "HEILKUNDEWUNDEN" => {
                Some(self.talent_heilkunde_wunden())
            }
            "Holzbearbeitung" | "holzbearbeitung" | "HOLZBEARBEITUNG" => {
                Some(self.talent_holzbearbeitung())
            }
            "Lebensmittelbearbeitung" | "lebensmittelbearbeitung" | "LEBENSMITTELBEARBEITUNG" => {
                Some(self.talent_lebensmittelbearbeitung())
            }
            "Lederbearbeitung" | "lederbearbeitung" | "LEDERBEARBEITUNG" => {
                Some(self.talent_lederbearbeitung())
            }
            "MalenZeichnen" | "malenzeichnen" | "MALENZEICHNEN" | "Malen" | "Zeichnen"
            | "malen" | "zeichnen" | "MALEN" | "ZEICHNEN" => Some(self.talent_malen_zeichnen()),
            "Metallbearbeitung" | "metallbearbeitung" | "METALLBEARBEITUNG" => {
                Some(self.talent_metallarbeitung())
            }
            "Musizieren" | "musizieren" | "MUSIZIEREN" => Some(self.talent_musizieren()),
            "Schloesserknacken" | "schloesserknacken" | "SCHLOESSERKNACKEN"
            | "Schlösserknacken" | "schlösserknacken" | "SCHLÖSSERKNACKEN" => {
                Some(self.talent_schloesserknacken())
            }
            "Steinbearbeitung" | "steinbearbeitung" | "STEINBEARBEITUNG" => {
                Some(self.talent_steinbearbeitung())
            }
            "Stoffbearbeitung" | "stoffbearbeitung" | "STOFFBEARBEITUNG" => {
                Some(self.talent_stoffbearbeitung())
            }
            _ => None,
        }
    }
    pub(crate) fn ini(&self) -> u8 {
        let d = Dice::new(6);
        let ini = (self.attribute_mu().value() + self.attribute_ge().value()) / 2;
        ini + d.roll()
    }
    pub(crate) fn check_talent(&self, id: &str, mod_value: u8, mod_op: Operator) -> String {
        let talent = match self.get_talent_by_id(id) {
            Some(tal) => tal,
            _ => return format!("You do not have the talent \"{}\"", id),
        };
        let mut current_talent_val = talent.value();
        let dummy = Attribute::new("DUMMY", 0);
        let attr1 = match self.get_attribute_by_id(talent.attribute_1_id()) {
            Some(attr) => attr,
            _ => &dummy,
        };
        let attr2 = match self.get_attribute_by_id(talent.attribute_2_id()) {
            Some(attr) => attr,
            _ => &dummy,
        };
        let attr3 = match self.get_attribute_by_id(talent.attribute_3_id()) {
            Some(attr) => attr,
            _ => &dummy,
        };
        let mut actual_attribute_value_1 = match self.get_attribute_by_id(talent.attribute_1_id()) {
            Some(attr) => attr.value(),
            _ => 0,
        };
        let mut actual_attribute_value_2 = match self.get_attribute_by_id(talent.attribute_2_id()) {
            Some(attr) => attr.value(),
            _ => 0,
        };
        let mut actual_attribute_value_3 = match self.get_attribute_by_id(talent.attribute_3_id()) {
            Some(attr) => attr.value(),
            _ => 0,
        };
        if mod_value > 0 && mod_op != Operator::NoP {
            if mod_op == Operator::Minus {
                actual_attribute_value_1 =
                    substract_without_overflow(actual_attribute_value_1, mod_value);
                actual_attribute_value_2 =
                    substract_without_overflow(actual_attribute_value_2, mod_value);
                actual_attribute_value_3 =
                    substract_without_overflow(actual_attribute_value_3, mod_value);
            } else {
                actual_attribute_value_1 =
                    add_without_overflow(actual_attribute_value_1, mod_value);
                actual_attribute_value_2 =
                    add_without_overflow(actual_attribute_value_2, mod_value);
                actual_attribute_value_3 =
                    add_without_overflow(actual_attribute_value_3, mod_value);
            }
        }
        let r1 = check_talent_roll(attr1, actual_attribute_value_1, &mut current_talent_val);
        let r2 = check_talent_roll(attr2, actual_attribute_value_2, &mut current_talent_val);
        let r3 = check_talent_roll(attr3, actual_attribute_value_3, &mut current_talent_val);
        let is_success = r1.is_success() && r2.is_success() && r3.is_success();
        let is_critical_success = is_success
            && (r1.is_critical_success() || r2.is_critical_success() || r3.is_critical_success());
        let is_critical_failure = !is_success
            && (r1.is_critical_failure() || r2.is_critical_failure() || r3.is_critical_failure());
        let check = match mod_op {
            Operator::Minus => format!("{}(-{})", talent.name(), mod_value),
            Operator::Plus => format!("{}(+{})", talent.name(), mod_value),
            Operator::NoP => talent.name().to_string(),
        };
        let msg = if is_critical_success {
            "Critical success"
        } else if is_success {
            "Success"
        } else if !is_success && !is_critical_failure && !is_critical_success {
            "Failure!"
        } else if !is_success && !is_critical_success && is_critical_failure {
            "Critical Failure!"
        } else {
            "Beep ... bop ... I'm not sure what happened, sorry!"
        };
        let mut q = get_quality(current_talent_val);
        if is_critical_success {
            if r1.is_critical_success() {
                q = add_without_overflow(q, r1.crit_count());
            }
            if r2.is_critical_success() {
                q = add_without_overflow(q, r2.crit_count());
            }
            if r3.is_critical_success() {
                q = add_without_overflow(q, r3.crit_count());
            }
        }
        if is_success {
            format!(
                ":white_check_mark: **{} - {} with quality level {}!**\n{}\n{}\n{}",
                check, msg, q, r1, r2, r3
            )
        } else {
            format!(":x: **{} - {}**\n{}\n{}\n{}", check, msg, r1, r2, r3)
        }
    }
}

fn get_quality(talent_left: u8) -> u8 {
    let q = talent_left / 3;
    if q == 0 {
        1
    } else if q >= 6 {
        6
    } else {
        q
    }
}

fn get_attribute_display(attribute: &Attribute, value: u8) -> String {
    match attribute.value().cmp(&value) {
        Ordering::Equal => format!("{} {}", attribute.name(), attribute.value()),
        Ordering::Less => format!(
            "{} {}(+{})",
            attribute.name(),
            attribute.value(),
            substract_without_overflow(value, attribute.value())
        ),
        Ordering::Greater => format!(
            "{} {}(-{})",
            attribute.name(),
            attribute.value(),
            substract_without_overflow(attribute.value(), value)
        ),
    }
}

fn check_talent_roll(
    attribute: &Attribute,
    attribute_val: u8,
    current_talent_val: &mut u8,
) -> TalentCheckResult {
    let d = Dice::new(20);
    let roll = d.roll();
    if roll == 1 {
        let crit_count = check_critical_success(attribute_val, 0);
        if crit_count > 0 {
            let s = format!(
                ":partying_face: - {}: {}, crit check succeded {} time(-s)",
                get_attribute_display(attribute, attribute_val),
                roll,
                crit_count
            );
            return TalentCheckResult::new(&s, ResultType::CriticalSuccess, crit_count);
        } else {
            let s = format!(
                ":green_circle: - {}: {}, crit check failed.",
                get_attribute_display(attribute, attribute_val),
                roll
            );
            return TalentCheckResult::new(&s, ResultType::Success, crit_count);
        }
    }
    if roll == 20 {
        let crit_count = check_critical_failure(attribute_val, 0);
        if crit_count > 0 {
            let s = format!(
                ":skull: - {}: {}, crit check failed {} time(-s)",
                get_attribute_display(attribute, attribute_val),
                roll,
                crit_count
            );
            return TalentCheckResult::new(&s, ResultType::CriticalFailure, 1);
        } else {
            let s = format!(
                ":red_circle: - {}: {}, crit check succeeded.",
                get_attribute_display(attribute, attribute_val),
                roll
            );
            return TalentCheckResult::new(&s, ResultType::Failure, 0);
        }
    }
    if roll <= attribute_val {
        let s = format!(
            ":green_circle: - {}: {}",
            get_attribute_display(attribute, attribute_val),
            roll
        );
        return TalentCheckResult::new(&s, ResultType::Success, 0);
    }
    if *current_talent_val < 1 {
        let s = format!(
            ":red_circle: - {}: {}",
            get_attribute_display(attribute, attribute_val),
            roll
        );
        return TalentCheckResult::new(&s, ResultType::Failure, 0);
    }
    let diff = substract_without_overflow(roll, attribute_val);
    if *current_talent_val < diff {
        let s = format!(
            ":red_circle: - {}: {}: {} points left, but {} missing.",
            get_attribute_display(attribute, attribute_val),
            roll,
            *current_talent_val,
            diff
        );
        TalentCheckResult::new(&s, ResultType::Failure, 0)
    } else {
        *current_talent_val = substract_without_overflow(*current_talent_val, diff);
        let s = format!(
            ":orange_circle: - {}: {}: {} points used, {} left.",
            get_attribute_display(attribute, attribute_val),
            roll,
            diff,
            *current_talent_val
        );
        TalentCheckResult::new(&s, ResultType::Success, 0)
    }
}

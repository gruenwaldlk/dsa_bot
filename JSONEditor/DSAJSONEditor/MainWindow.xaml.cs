using System;
using System.Collections.Generic;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;
using System.IO;
using Newtonsoft.Json;
using Microsoft.Win32;
using System.Text.RegularExpressions;

namespace DSAJSONEditor
{
    /// <summary>
    /// Interaktionslogik für MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        private Character character;
        List<Talent> talente_list1_1, talente_list1_2, talente_list2_1, talente_list2_2;
        public MainWindow()
        {
            character = new Character();
            InitializeComponent();
            populateFields();
        }
        private void LoadJson_Click(object sender, RoutedEventArgs e)
        {
            OpenFileDialog openFileDialog = new OpenFileDialog();
            openFileDialog.Filter = "json files (*.json)|*.json|All files (*.*)|*.*";
            openFileDialog.FilterIndex = 1;
            openFileDialog.RestoreDirectory = true;
            if (openFileDialog.ShowDialog() == true)
            {
                using (StreamReader r = new StreamReader(openFileDialog.FileName))
                {
                    string json = r.ReadToEnd();
                    character = JsonConvert.DeserializeObject<Character>(json);
                }
            }
            populateFields();
        }
        private void SaveJson_Click(object sender, RoutedEventArgs e)
        {
            //read from character page
            character.name = Value_Name.Text;
            int tmp_ch = Int16.Parse(Value_CH.Text);
            int tmp_ff = Int16.Parse(Value_FF.Text);
            int tmp_ge = Int16.Parse(Value_GE.Text);
            int tmp_in = Int16.Parse(Value_IN.Text);
            int tmp_kk = Int16.Parse(Value_KK.Text);
            int tmp_kl = Int16.Parse(Value_KL.Text);
            int tmp_ko = Int16.Parse(Value_KO.Text);

            if (0 < tmp_ch && tmp_ch < 255) {
                character.attribute_ch.value = tmp_ch;
            }
            if (0 < tmp_ff && tmp_ff < 255)
            {
                character.attribute_ff.value = tmp_ff;
            }
            if (0 < tmp_ge && tmp_ge < 255)
            {
                character.attribute_ge.value = tmp_ge;
            }
            if (0 < tmp_in && tmp_in < 255)
            {
                character.attribute_in.value = tmp_in;
            }
            if (0 < tmp_kk && tmp_kk < 255)
            {
                character.attribute_kk.value = tmp_kk;
            }
            if (0 < tmp_kl && tmp_kl < 255)
            {
                character.attribute_kl.value = tmp_kl;
            }
            if (0 < tmp_ko && tmp_ko < 255)
            {
                character.attribute_ko.value = tmp_ko;
            }
            
            //talents page are handled automatically
            Stream myStream;
            SaveFileDialog saveFileDialog = new SaveFileDialog();
            saveFileDialog.Filter = "json files (*.json)|*.json|All files (*.*)|*.*";
            saveFileDialog.FilterIndex = 1;
            saveFileDialog.RestoreDirectory = true;

            if (saveFileDialog.ShowDialog() == true)
            {
                if (!saveFileDialog.FileName.EndsWith(".json"))
                {
                    saveFileDialog.FileName = saveFileDialog.FileName + ".json";
                }
                if ((myStream = saveFileDialog.OpenFile()) != null)
                {
                    StreamWriter sw = new StreamWriter(myStream);
                    String output = JsonConvert.SerializeObject(character);
                    sw.Write(output);
                    sw.Flush();
                    myStream.Close();
                }
            }
        }
        private void NewCharacter_Click(object sender, RoutedEventArgs e)
        {
            character = new Character();
            populateFields();
        }
        private void populateFields()
        {
            //Populate Character page
            //display texts
            {
                TextBlock txtName = new TextBlock();
                txtName.Text = "Name";
                txtName.FontSize = 18;
                txtName.FontWeight = FontWeights.Bold;
                CharacterTab.Children.Add(txtName);
                Grid.SetColumn(txtName, 1);
                Grid.SetRow(txtName, 1);

                TextBlock txt_ch = new TextBlock();
                txt_ch.Text = character.attribute_ch.name;
                txt_ch.FontSize = 18;
                txt_ch.FontWeight = FontWeights.Bold;
                CharacterTab.Children.Add(txt_ch);
                Grid.SetColumn(txt_ch, 1);
                Grid.SetRow(txt_ch, 3);

                TextBlock txt_ff = new TextBlock();
                txt_ff.Text = character.attribute_ff.name;
                txt_ff.FontSize = 18;
                txt_ff.FontWeight = FontWeights.Bold;
                CharacterTab.Children.Add(txt_ff);
                Grid.SetColumn(txt_ff, 1);
                Grid.SetRow(txt_ff, 4);

                TextBlock txt_ge = new TextBlock();
                txt_ge.Text = character.attribute_ge.name;
                txt_ge.FontSize = 18;
                txt_ge.FontWeight = FontWeights.Bold;
                CharacterTab.Children.Add(txt_ge);
                Grid.SetColumn(txt_ge, 1);
                Grid.SetRow(txt_ge, 5);

                TextBlock txt_in = new TextBlock();
                txt_in.Text = character.attribute_in.name;
                txt_in.FontSize = 18;
                txt_in.FontWeight = FontWeights.Bold;
                CharacterTab.Children.Add(txt_in);
                Grid.SetColumn(txt_in, 1);
                Grid.SetRow(txt_in, 6);


                TextBlock txt_kk = new TextBlock();
                txt_kk.Text = character.attribute_kk.name;
                txt_kk.FontSize = 18;
                txt_kk.FontWeight = FontWeights.Bold;
                CharacterTab.Children.Add(txt_kk);
                Grid.SetColumn(txt_kk, 1);
                Grid.SetRow(txt_kk, 7);


                TextBlock txt_kl = new TextBlock();
                txt_kl.Text = character.attribute_kl.name;
                txt_kl.FontSize = 18;
                txt_kl.FontWeight = FontWeights.Bold;
                CharacterTab.Children.Add(txt_kl);
                Grid.SetColumn(txt_kl, 1);
                Grid.SetRow(txt_kl, 8);


                TextBlock txt_ko = new TextBlock();
                txt_ko.Text = character.attribute_ko.name;
                txt_ko.FontSize = 18;
                txt_ko.FontWeight = FontWeights.Bold;
                CharacterTab.Children.Add(txt_ko);
                Grid.SetColumn(txt_ko, 1);
                Grid.SetRow(txt_ko, 9);
            }
            // display values
            Value_Name.Text = character.name;
            Value_CH.Text = character.attribute_ch.value.ToString();
            Value_FF.Text = character.attribute_ff.value.ToString();
            Value_GE.Text = character.attribute_ge.value.ToString();
            Value_IN.Text = character.attribute_in.value.ToString();
            Value_KK.Text = character.attribute_kk.value.ToString();
            Value_KL.Text = character.attribute_kl.value.ToString();
            Value_KO.Text = character.attribute_ko.value.ToString();

            // populate talent pages
            talente_list1_1 = new List<Talent>();
            talente_list2_1 = new List<Talent>();
            talente_list1_2 = new List<Talent>();
            talente_list2_2 = new List<Talent>();

            talente_list1_1.Add(character.talent_alchimie);
            talente_list1_1.Add(character.talent_bekehren_ueberzeugen);
            talente_list1_1.Add(character.talent_betoeren);
            talente_list1_1.Add(character.talent_boote);
            talente_list1_1.Add(character.talent_brettspiel);
            talente_list1_1.Add(character.talent_einschuechtern);
            talente_list1_1.Add(character.talent_etikette);
            talente_list1_1.Add(character.talent_faehrtensuchen);
            talente_list1_1.Add(character.talent_fahrzeuge);
            talente_list1_1.Add(character.talent_fesseln);
            talente_list1_1.Add(character.talent_fischen_angeln);
            talente_list1_1.Add(character.talent_fliegen);
            talente_list1_1.Add(character.talent_gassenwissen);
            talente_list1_1.Add(character.talent_gaukeleien);
            talente_list1_1.Add(character.talent_geographie);

            talente_list1_2.Add(character.talent_geschichtswissen);
            talente_list1_2.Add(character.talent_goetter_kulte);
            talente_list1_2.Add(character.talent_handel);
            talente_list1_2.Add(character.talent_heilkunde_gift);
            talente_list1_2.Add(character.talent_heilkunde_krankheiten);
            talente_list1_2.Add(character.talent_heilkunde_seele);
            talente_list1_2.Add(character.talent_heilkunde_wunden);
            talente_list1_2.Add(character.talent_holzbearbeitung);
            talente_list1_2.Add(character.talent_klettern);
            talente_list1_2.Add(character.talent_koerperbeherrschung);
            talente_list1_2.Add(character.talent_kraftakt);
            talente_list1_2.Add(character.talent_kriegskunst);
            talente_list1_2.Add(character.talent_lebensmittelbearbeitung);
            talente_list1_2.Add(character.talent_lederbearbeitung);
            talente_list1_2.Add(character.talent_magiekunde);

            talente_list2_1.Add(character.talent_malen_zeichnen);
            talente_list2_1.Add(character.talent_mechanik);
            talente_list2_1.Add(character.talent_menschenkenntnis);
            talente_list2_1.Add(character.talent_metallarbeitung);
            talente_list2_1.Add(character.talent_musizieren);
            talente_list2_1.Add(character.talent_orientierung);
            talente_list2_1.Add(character.talent_pflanzenkunde);
            talente_list2_1.Add(character.talent_rechnen);
            talente_list2_1.Add(character.talent_rechtskunde);
            talente_list2_1.Add(character.talent_reiten);
            talente_list2_1.Add(character.talent_sagen_legenden);
            talente_list2_1.Add(character.talent_schloesserknacken);
            talente_list2_1.Add(character.talent_schwimmen);
            talente_list2_1.Add(character.talent_selbstbeherrschung);
            talente_list2_1.Add(character.talent_singen);

            talente_list2_2.Add(character.talent_sinnesschaerfe);
            talente_list2_2.Add(character.talent_sphaerenkunde);
            talente_list2_2.Add(character.talent_steinbearbeitung);
            talente_list2_2.Add(character.talent_sternkunde);
            talente_list2_2.Add(character.talent_stoffbearbeitung);
            talente_list2_2.Add(character.talent_tanzen);
            talente_list2_2.Add(character.talent_taschendiebstahl);
            talente_list2_2.Add(character.talent_tierkunde);
            talente_list2_2.Add(character.talent_ueberreden);
            talente_list2_2.Add(character.talent_verbergen);
            talente_list2_2.Add(character.talent_verkleiden);
            talente_list2_2.Add(character.talent_wildnisleben);
            talente_list2_2.Add(character.talent_wilenskraft);
            talente_list2_2.Add(character.talent_zechen);

            Talente1_1.ItemsSource = null;
            Talente1_2.ItemsSource = null;
            Talente2_1.ItemsSource = null;
            Talente2_2.ItemsSource = null;
            Talente1_1.ItemsSource = talente_list1_1;
            Talente1_2.ItemsSource = talente_list1_2;
            Talente2_1.ItemsSource = talente_list2_1;
            Talente2_2.ItemsSource = talente_list2_2;

        }
        private void NumberValidationTextBox(object sender, TextCompositionEventArgs e)
        {
            Regex regex = new Regex("[^0-9]+");
            e.Handled = regex.IsMatch(e.Text);
        }
        private void TabControl_SelectionChanged(object sender, SelectionChangedEventArgs e)
        {
            //do nothing
        }
        private void ExitWindow_Click(object sender, RoutedEventArgs e) {
            this.Close();
        }
    }

    //jasses for (de)serializing the json
    public class Character
    {
        public String name;
        public UInt64 owner;
        public Attribute attribute_ch;
        public Attribute attribute_ff;
        public Attribute attribute_ge;
        public Attribute attribute_in;
        public Attribute attribute_kk;
        public Attribute attribute_kl;
        public Attribute attribute_ko;
        public Attribute attribute_mu; public Talent talent_alchimie { get; set; }
        public Talent talent_bekehren_ueberzeugen { get; set; }
        public Talent talent_betoeren { get; set; }
        public Talent talent_boote { get; set; }
        public Talent talent_brettspiel { get; set; }
        public Talent talent_einschuechtern { get; set; }
        public Talent talent_etikette { get; set; }
        public Talent talent_faehrtensuchen { get; set; }
        public Talent talent_fahrzeuge { get; set; }
        public Talent talent_fesseln { get; set; }
        public Talent talent_fischen_angeln { get; set; }
        public Talent talent_fliegen { get; set; }
        public Talent talent_gassenwissen { get; set; }
        public Talent talent_gaukeleien { get; set; }
        public Talent talent_geographie { get; set; }
        public Talent talent_geschichtswissen { get; set; }
        public Talent talent_goetter_kulte { get; set; }
        public Talent talent_handel { get; set; }
        public Talent talent_heilkunde_gift { get; set; }
        public Talent talent_heilkunde_krankheiten { get; set; }
        public Talent talent_heilkunde_seele { get; set; }
        public Talent talent_heilkunde_wunden { get; set; }
        public Talent talent_holzbearbeitung { get; set; }
        public Talent talent_klettern { get; set; }
        public Talent talent_koerperbeherrschung { get; set; }
        public Talent talent_kraftakt { get; set; }
        public Talent talent_kriegskunst { get; set; }
        public Talent talent_lebensmittelbearbeitung { get; set; }
        public Talent talent_lederbearbeitung { get; set; }
        public Talent talent_magiekunde { get; set; }
        public Talent talent_malen_zeichnen { get; set; }
        public Talent talent_mechanik { get; set; }
        public Talent talent_menschenkenntnis { get; set; }
        public Talent talent_metallarbeitung { get; set; }
        public Talent talent_musizieren { get; set; }
        public Talent talent_orientierung { get; set; }
        public Talent talent_pflanzenkunde { get; set; }
        public Talent talent_rechnen { get; set; }
        public Talent talent_rechtskunde { get; set; }
        public Talent talent_reiten { get; set; }
        public Talent talent_sagen_legenden { get; set; }
        public Talent talent_schloesserknacken { get; set; }
        public Talent talent_schwimmen { get; set; }
        public Talent talent_selbstbeherrschung { get; set; }
        public Talent talent_singen { get; set; }
        public Talent talent_sinnesschaerfe { get; set; }
        public Talent talent_sphaerenkunde { get; set; }
        public Talent talent_steinbearbeitung { get; set; }
        public Talent talent_sternkunde { get; set; }
        public Talent talent_stoffbearbeitung { get; set; }
        public Talent talent_tanzen { get; set; }
        public Talent talent_taschendiebstahl { get; set; }
        public Talent talent_tierkunde { get; set; }
        public Talent talent_ueberreden { get; set; }
        public Talent talent_verbergen { get; set; }
        public Talent talent_verkleiden { get; set; }
        public Talent talent_wildnisleben { get; set; }
        public Talent talent_wilenskraft { get; set; }
        public Talent talent_zechen { get; set; }
        public Character()
        {
            name = "";
            owner = 0;

            attribute_ch = new Attribute("CH", 0);
            attribute_ff = new Attribute("FF", 0);
            attribute_ge = new Attribute("GE", 0);
            attribute_in = new Attribute("IN", 0);
            attribute_kk = new Attribute("KK", 0);
            attribute_kl = new Attribute("KL", 0);
            attribute_ko = new Attribute("KO", 0);
            attribute_mu = new Attribute("MU", 0);

            talent_alchimie = new Talent("MU", "KL", "FF", "Alchimie", 0);
            talent_bekehren_ueberzeugen = new Talent("MU", "KL", "CH", "Bekehren & Überzeugen", 0);
            talent_betoeren = new Talent("MU", "CH", "CH", "Betoeren", 0);
            talent_boote = new Talent("FF", "GE", "KK", "Boote", 0);
            talent_brettspiel = new Talent("KL", "KL", "IN", "Brett- & Glücksspiel", 0);
            talent_einschuechtern = new Talent("MU", "IN", "CH", "Einschüchtern", 0);
            talent_etikette = new Talent("KL", "IN", "CH", "Etikette", 0);
            talent_faehrtensuchen = new Talent("MU", "IN", "GE", "Fährtensuchen", 0);
            talent_fahrzeuge = new Talent("CH", "FF", "KO", "Fahrzeuge", 0);
            talent_fesseln = new Talent("KL", "FF", "KK", "Fesseln", 0);
            talent_fischen_angeln = new Talent("FF", "GE", "KO", "Fischen & Angeln", 0);
            talent_fliegen = new Talent("MU", "IN", "GE", "Fliegen", 0);
            talent_gassenwissen = new Talent("KL", "IN", "CH", "Gassenwissen", 0);
            talent_gaukeleien = new Talent("MU", "CH", "FF", "Gaukelei", 0);
            talent_geographie = new Talent("KL", "KL", "IN", "Geographie", 0);
            talent_geschichtswissen = new Talent("KL", "KL", "IN", "Geschichtswissen", 0);
            talent_goetter_kulte = new Talent("KL", "KL", "IN", "Götter & Kulte", 0);
            talent_handel = new Talent("KL", "IN", "CH", "Handel", 0);
            talent_heilkunde_gift = new Talent("MU", "KL", "IN", "Heilkunde: Gift", 0);
            talent_heilkunde_krankheiten = new Talent("MU", "IN", "KO", "Heilkunde: Krankheiten", 0);
            talent_heilkunde_seele = new Talent("IN", "CH", "KO", "Heilkunde: Seele", 0);
            talent_heilkunde_wunden = new Talent("KL", "FF", "FF", "Heilkunde: Wunden", 0);
            talent_holzbearbeitung = new Talent("FF", "GE", "KK", "Holzbearbeitung", 0);
            talent_klettern = new Talent("MU", "GE", "KK", "Klettern", 0);
            talent_koerperbeherrschung = new Talent("GE", "GE", "KO", "Körperbeherrschung", 0);
            talent_kraftakt = new Talent("KO", "KK", "KK", "Kraftakt", 0);
            talent_kriegskunst = new Talent("MU", "KL", "IN", "Kriegskunst", 0);
            talent_lebensmittelbearbeitung = new Talent("IN", "FF", "FF", "Lebensmittelbearbeitung", 0);
            talent_lederbearbeitung = new Talent("FF", "GE", "KO", "Lederbearbeitung", 0);
            talent_magiekunde = new Talent("KL", "KL", "IN", "Magiekunde", 0);
            talent_malen_zeichnen = new Talent("IN", "FF", "FF", "Malen & Zeichnen", 0);
            talent_mechanik = new Talent("KL", "KL", "FF", "Mechanik", 0);
            talent_menschenkenntnis = new Talent("KL", "IN", "CH", "Menschenkenntniss", 0);
            talent_metallarbeitung = new Talent("FF", "KO", "KK", "Metallbearbeitung", 0);
            talent_musizieren = new Talent("CH", "FF", "KO", "Musizieren", 0);
            talent_orientierung = new Talent("KL", "IN", "IN", "Orientierung", 0);
            talent_pflanzenkunde = new Talent("KL", "FF", "KO", "Pflanzenkunde", 0);
            talent_rechnen = new Talent("KL", "KL", "IN", "Rechnen", 0);
            talent_rechtskunde = new Talent("KL", "KL", "IN", "Rechtskunde", 0);
            talent_reiten = new Talent("CH", "GE", "KK", "Reiten", 0);
            talent_sagen_legenden = new Talent("KL", "KL", "IN", "Sagen & Legenden", 0);
            talent_schloesserknacken = new Talent("IN", "FF", "FF", "Schlösserknacken", 0);
            talent_schwimmen = new Talent("GE", "KO", "KK", "Schwimmen", 0);
            talent_selbstbeherrschung = new Talent("MU", "MU", "KO", "Selbstbeherrschung", 0);
            talent_singen = new Talent("KL", "CH", "KO", "Singen", 0);
            talent_sinnesschaerfe = new Talent("KL", "IN", "IN", "Sinnesschärfe", 0);
            talent_sphaerenkunde = new Talent("KL", "KL", "IN", "Sphärenkunde", 0);
            talent_steinbearbeitung = new Talent("FF", "FF", "KK", "Steinbearbeitung", 0);
            talent_sternkunde = new Talent("KL", "KL", "IN", "Sternkunde", 0);
            talent_stoffbearbeitung = new Talent("KL", "FF", "FF", "Stoffbearbeitung", 0);
            talent_tanzen = new Talent("KL", "CH", "GE", "Tanzen", 0);
            talent_taschendiebstahl = new Talent("MU", "FF", "GE", "Taschendiebstahl", 0);
            talent_tierkunde = new Talent("MU", "MU", "CH", "Tierkunde", 0);
            talent_ueberreden = new Talent("MU", "IN", "CH", "Überreden", 0);
            talent_verbergen = new Talent("MU", "IN", "GE", "Verbergen", 0);
            talent_verkleiden = new Talent("IN", "CH", "GE", "Verkleiden", 0);
            talent_wildnisleben = new Talent("MU", "GE", "KO", "Wildnisleben", 0);
            talent_wilenskraft = new Talent("MU", "IN", "CH", "Willenskraft", 0);
            talent_zechen = new Talent("KL", "KO", "KK", "Zechen", 0);
        }
    }
    public class Attribute
    {
        public String name { get; set; }
        public int value { get; set; }
        public Attribute(String in_name, int in_value)
        {
            name = in_name;
            value = in_value;
        }
    }
    public class Talent
    {
        public String attribute_1_id;
        public String attribute_2_id;
        public String attribute_3_id;
        public String name { get; set; }
        public int value { get; set; }

        public Talent(string attribute_1_id, string attribute_2_id, string attribute_3_id, string name, int value)
        {
            this.attribute_1_id = attribute_1_id;
            this.attribute_2_id = attribute_2_id;
            this.attribute_3_id = attribute_3_id;
            this.name = name;
            this.value = value;
        }
    }

}

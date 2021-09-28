//! 
//! ## NAME
//!   Safe Network - hammer
//!   Version: 0.0.3
//! 
//! ## SUMMARY
//!   A simple front end to the Safe Network.
//! 
//! ## Basic Usage
//!		$ hammer
//! 
//! ### Optional: language code ISO639-1, defaults to en.
//! 	$ hammer el
//! 	$ hammer en
//! 	$ hammer ko
//! 
//! #### Options:
//!     -h, --help          display help and exit
//!     -V, --version       output version information and exit
//! 
//! ## DESCRIPTION
//!
//!   This is a simple front end GUI for the Safe Network and makes calls to the Safe Network Command Line Interface (CLI).
//!
//!   For more about the Safe Network see:
//!
//!   <https://safenetwork.tech/>
//!
//!   <https://safenetforum.org/>
//!
//!   <https://github.com/maidsafe/>
//! 
//! ## AUTHOR
//!
//!   David P Brown
//! 
//! ## REPORTING BUGS
//!
//!   Report bugs to <mailto://bugs@davidpbrown.co.uk>
//! 
//! ## COPYRIGHT
//!
//!   Copyleft!
//!
//! ## Known issues:
//!
//!   Errors from the network come with colour via color_eyre and then when put to a log file, those create clutter with escape sequences.
//!
//! ## Expected behaviour:
//!
//!   User input is cleaned with Rust's ammonia::clean_text but otherwise the request is put to the CLI as the user asks it.
//!
//!   The actions requested will continue in background Safe CLI processes, even when the GUI and terminal have been closed. Downloads and log files, will then continue to receive the outputs of requests from the network via those safe processes.
//!
// TO DO
//
// ## todo later: Tidy docs and readme
// ## todo later: how to capture success to note that to log.txt ??
// ## todo later: check in case get cat etc for files naturally spawn a useful filename - save forcing the filename
// ## todo later: check how cat reacts to dir - validate input as file or dir - would need to create dir?
// ## todo later: check nrs add behaviour on "" and add in for this?.. or does it just work (not None = cancel)??
// ## todo later: authenticate

// ## Modules and Crates ##
extern crate getopts;
extern crate whoami;

extern crate image;
extern crate base64;
extern crate chrono;
extern crate ammonia;

extern crate grep_regex;
extern crate grep_searcher;
extern crate grep_matcher;
extern crate grep_printer;
extern crate sedregex;

extern crate fltk;

pub mod cli_args;
pub mod galleries;

pub mod img_safe_logo;
pub mod img_dragon;
pub mod img_venn_psf;
pub mod check_dir_exists;
pub mod iso8601;
pub mod user_settings;

use fltk::{prelude::*, app,	window::Window};

use galleries::draw_gallery_af;
use galleries::draw_gallery_am;
use galleries::draw_gallery_ar;
use galleries::draw_gallery_be;
use galleries::draw_gallery_bg;
use galleries::draw_gallery_bn;
use galleries::draw_gallery_ca;
use galleries::draw_gallery_cs;
use galleries::draw_gallery_da;
use galleries::draw_gallery_de;
use galleries::draw_gallery_el;
use galleries::draw_gallery;
use galleries::draw_gallery_es;
use galleries::draw_gallery_fa;
use galleries::draw_gallery_fi;
use galleries::draw_gallery_fr;
use galleries::draw_gallery_gl;
use galleries::draw_gallery_he;
use galleries::draw_gallery_hi;
use galleries::draw_gallery_hr;
use galleries::draw_gallery_ht;
use galleries::draw_gallery_hu;
use galleries::draw_gallery_hy;
use galleries::draw_gallery_id;
use galleries::draw_gallery_it;
use galleries::draw_gallery_ja;
use galleries::draw_gallery_kn;
use galleries::draw_gallery_ko;
use galleries::draw_gallery_ku;
use galleries::draw_gallery_lt;
use galleries::draw_gallery_ml;
use galleries::draw_gallery_mr;
use galleries::draw_gallery_ms;
use galleries::draw_gallery_nl;
use galleries::draw_gallery_no;
use galleries::draw_gallery_pl;
use galleries::draw_gallery_pt;
use galleries::draw_gallery_ro;
use galleries::draw_gallery_ru;
use galleries::draw_gallery_sk;
use galleries::draw_gallery_sr;
use galleries::draw_gallery_sv;
use galleries::draw_gallery_sw;
use galleries::draw_gallery_ta;
use galleries::draw_gallery_te;
use galleries::draw_gallery_th;
use galleries::draw_gallery_tk;
use galleries::draw_gallery_tr;
use galleries::draw_gallery_tt;
use galleries::draw_gallery_ug;
use galleries::draw_gallery_uk;
use galleries::draw_gallery_ur;
use galleries::draw_gallery_vi;
use galleries::draw_gallery_xh;
use galleries::draw_gallery_zh_traditional;
use galleries::draw_gallery_zh;

// ## fn main ##
fn main() {
//!   Safe GUI
//!
//! 	Accepts iso639-1 as an arguement for preferred language
//! 	or otherwise queries the operating system for the user's first language.
//!
//!		Creates then a window for the GUI content.
//!

// ## set language ##
	let first_lang: String;

	let args: Vec<String> = cli_args::get_args();
	if args.len() == 1 {
// ## get lang - from arguement ##
		first_lang = format!("{}", &args[0]);
	} else { 
// ## get lang - from user prefered language ##
		let languages: Vec<String> = whoami::lang().collect::<Vec<String>>();
		first_lang = format!("{}", &languages[0]);
	};
// ## lang
	let mut ln = &first_lang[..2];

	if ln == "zh" { 
		let lan = &first_lang[..];
		ln = match lan {
			"zh-cn" => { "zh_traditional" },
			"zh-hk" => { "zh_traditional" },
			"zh-mo" => { "zh_traditional" },
			"zh-sg" => { "zh_traditional" },
			"zh-tw" => { "zh_traditional" },
			_ => { "zh" },
		};
	};
// ## /set language ##

// ## /match lang ##
	match ln {
"af" => { println!("\nVeilige netwerk - hamer\n\nEenvoudig Grafiese gebruikerskoppelvlak vir die Veilige netwerk\n\n# Versoeke aan die Veilige netwerk word gemaak via die Veilige CLI\n#\n# Suksesvolle aflaai word in die gids geplaas ./Aflaaie/\n# saam met logboeke en enige foute van die netwerk.\n#\n# As die Safe CLI nie beskikbaar is nie, kan foute hier verskyn.\n\n--------------\ntoetsnetwerk\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Veilige netwerk")  .center_screen();  draw_gallery_af();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "am" => { println!("\nደህንነቱ የተጠበቀ አውታረ መረብ - መዶሻ\n\nቀላል ግራፊክ የተጠቃሚ በይነገጽ ለ ደህንነቱ የተጠበቀ አውታረ መረብ\n\n# ጥያቄዎች ለ ደህንነቱ የተጠበቀ አውታረ መረብ በኩል የተሰሩ ናቸው ደህንነቱ የተጠበቀ CLI\n#\n# የተሳካ ውርዶች በአቃፊው ውስጥ ይቀመጣሉ ./ውርዶች/\n# ከምዝግብ ማስታወሻዎች እና ከአውታረ መረቡ ማናቸውም ስህተቶች ጋር።\n#\n# ደህንነቱ የተጠበቀ CLI ሊገኝ የማይችል ከሆነ ፣ ስህተቶች እዚህ ሊታዩ ይችላሉ።\n\n--------------\nየሙከራ አውታረ መረብ\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("ደህንነቱ የተጠበቀ አውታረ መረብ")  .center_screen();  draw_gallery_am();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ar" => { println!("\nشبكة آمنة - شاكوش\n\nبسيط واجهة المستخدم الرسومية ل شبكة آمنة\n\n# طلبات إلى شبكة آمنة تتم عبر CLI آمن\n#\n# يتم وضع التنزيلات الناجحة في المجلد ./التحميلات/\n# إلى جانب السجلات وأية أخطاء من الشبكة.\n#\n# إذا لم يكن Safe CLI متوفرًا ، فقد تظهر الأخطاء هنا.\n\n--------------\nاختبار الشبكة\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("شبكة آمنة")  .center_screen();  draw_gallery_ar();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "be" => { println!("\nБяспечная сетка - малаток\n\nПросты Графічны карыстацкі інтэрфейс для Бяспечная сетка\n\n# Запыты да Бяспечная сетка вырабляюцца праз Бяспечны CLI\n#\n# Паспяховыя загрузкі змяшчаюцца ў тэчку ./Загрузкі/\n# разам з часопісамі і любымі памылкамі з сеткі.\n#\n# Калі бяспечны CLI недаступны, тут могуць паказацца памылкі.\n\n--------------\nтэставая сетка\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Бяспечная сетка")  .center_screen();  draw_gallery_be();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "bg" => { println!("\nБезопасна мрежа - чук\n\nПрост Графичен потребителски интерфейс за Безопасна мрежа\n\n# Искания до Безопасна мрежа се правят чрез Безопасен CLI\n#\n# Успешните изтегляния се поставят в папката ./Изтегляния/\n# заедно с регистрационни файлове и всякакви грешки от мрежата.\n#\n# Ако безопасният CLI не е наличен, тук може да се покажат грешки.\n\n--------------\nтестова мрежа\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Безопасна мрежа")  .center_screen();  draw_gallery_bg();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "bn" => { println!("\nনিরাপদ নেটওয়ার্ক - হাতুড়ি\n\nসরল গ্রাফিকাল ইউজার ইন্টারফেস জন্য নিরাপদ নেটওয়ার্ক\n\n# কাছে অনুরোধ নিরাপদ নেটওয়ার্ক এর মাধ্যমে তৈরি করা হয় নিরাপদ CLI\n#\n# সফল ডাউনলোড ফোল্ডারে রাখা হয় ./ডাউনলোড/\n# লগ এবং নেটওয়ার্ক থেকে কোন ত্রুটি সহ।\n#\n# যদি নিরাপদ CLI পাওয়া যায় না, ত্রুটিগুলি এখানে প্রদর্শিত হতে পারে।\n\n--------------\nটেস্ট নেটওয়ার্ক\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("নিরাপদ নেটওয়ার্ক")  .center_screen();  draw_gallery_bn();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ca" => { println!("\nXarxa segura - martell\n\nSenzill Interfaç gràfica d'usuari per al Xarxa segura\n\n# Sol·licituds a Xarxa segura es fan mitjançant el fitxer CLI segura\n#\n# Les baixades amb èxit es posen a la carpeta ./Descàrregues/\n# juntament amb els registres i qualsevol error de la xarxa.\n#\n# Si la CLI segura no està disponible, es poden mostrar errors aquí.\n\n--------------\nxarxa de proves\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Xarxa segura")  .center_screen();  draw_gallery_ca();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "cs" => { println!("\nBezpečná síť - kladivo\n\nJednoduchý Grafické uživatelské prostředí pro Bezpečná síť\n\n# Žádosti na Bezpečná síť jsou vyrobeny prostřednictvím Bezpečné CLI\n#\n# Úspěšné stahování je vloženo do složky ./Soubory ke stažení/\n# spolu s protokoly a všemi chybami ze sítě.\n#\n# Pokud Safe CLI není k dispozici, mohou se zde zobrazit chyby.\n\n--------------\ntestovací síť\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Bezpečná síť")  .center_screen();  draw_gallery_cs();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "da" => { println!("\nSikkert netværk - Hammer\n\nEnkel Grafisk brugerflade for Sikkert netværk\n\n# Anmodninger til Sikkert netværk laves via Sikker CLI\n#\n# Vellykkede downloads sættes i mappen ./Downloads/\n# sammen med logfiler og eventuelle fejl fra netværket.\n#\n# Hvis Safe CLI ikke er tilgængelig, vises der muligvis fejl her.\n\n--------------\ntestnetværk\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Sikkert netværk")  .center_screen();  draw_gallery_da();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "de" => { println!("\nSicheres Netzwerk - Hammer\n\nEinfach Grafische Benutzeroberfläche für die Sicheres Netzwerk\n\n# Anfragen an die Sicheres Netzwerk werden über die Sicheres CLI\n#\n# Erfolgreiche Downloads werden in den Ordner abgelegt ./Downloads/\n# zusammen mit Protokollen und Fehlern aus dem Netzwerk.\n#\n# Wenn die Safe CLI nicht verfügbar ist, können hier Fehler angezeigt werden.\n\n--------------\ntestnetzwerk\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Sicheres Netzwerk")  .center_screen();  draw_gallery_de();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "el" => { println!("\nΑσφαλές Δίκτυο - σφυρί\n\nΑπλός Γραφικό περιβάλλον διεπαφής χρήστη για το Ασφαλές Δίκτυο\n\n# Αιτήματα προς το Ασφαλές Δίκτυο γίνονται μέσω του Ασφαλές CLI\n#\n# Οι επιτυχημένες λήψεις τοποθετούνται στο φάκελο ./Λήψεις/\n# μαζί με αρχεία καταγραφής και τυχόν σφάλματα από το δίκτυο.\n#\n# Εάν το Safe CLI δεν είναι διαθέσιμο, ενδέχεται να εμφανιστούν σφάλματα εδώ.\n\n--------------\nδίκτυο δοκιμών\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Ασφαλές Δίκτυο")  .center_screen();  draw_gallery_el();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "es" => { println!("\nRed Segura - martillo\n\nSencillo Interfaz gráfica del usuario para el Red Segura\n\n# Solicitudes al Red Segura se hacen a través del CLI seguro\n#\n# Las descargas exitosas se colocan en la carpeta ./Descargas/\n# junto con los registros y cualquier error de la red.\n#\n# Si la CLI segura no está disponible, es posible que aparezcan errores aquí.\n\n--------------\nred de prueba\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Red Segura")  .center_screen();  draw_gallery_es();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "fa" => { println!("\nشبکه ایمن - چکش\n\nساده رابط کاربر گرافیکی برای شبکه ایمن\n\n# درخواست ها به شبکه ایمن از طریق CLI ایمن\n#\n# بارگیری های موفق در پوشه قرار می گیرند ./بارگیری ها/\n# همراه با گزارشات و هر گونه خطا از شبکه.\n#\n# اگر CLI ایمن در دسترس نیست ، ممکن است خطاها در اینجا نشان داده شود.\n\n--------------\nشبکه تست\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("شبکه ایمن")  .center_screen();  draw_gallery_fa();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "fi" => { println!("\nTurvallinen verkko - vasara\n\nYksinkertainen Graafinen käyttöliittymä varten Turvallinen verkko\n\n# Pyynnöt Turvallinen verkko tehdään kautta Turvallinen CLI\n#\n# Onnistuneet lataukset lisätään kansioon ./Lataukset/\n# sekä lokit ja mahdolliset verkon virheet.\n#\n# Jos Safe CLI ei ole käytettävissä, tässä saattaa näkyä virheitä.\n\n--------------\ntestiverkko\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Turvallinen verkko")  .center_screen();  draw_gallery_fi();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "fr" => { println!("\nRéseau sûr - marteau\n\nSimple Interface utilisateur graphique pour le Réseau sûr\n\n# Demandes au Réseau sûr se font via le CLI sécurisée\n#\n# Les téléchargements réussis sont placés dans le dossier ./Téléchargements/\n# ainsi que les journaux et toutes les erreurs du réseau.\n#\n# Si la Safe CLI n'est pas disponible, des erreurs peuvent s'afficher ici.\n\n--------------\nréseau de test\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Réseau sûr")  .center_screen();  draw_gallery_fr();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "gl" => { println!("\nRede segura - martelo\n\nSinxelo Interface gráfica de usuario para o Rede segura\n\n# Solicitudes ao Rede segura fanse a través do CLI seguro\n#\n# As descargas realizadas con éxito colócanse no cartafol ./Descargas/\n# xunto cos rexistros e calquera erro da rede.\n#\n# Se a CLI segura non está dispoñible, poden aparecer erros aquí.\n\n--------------\nrede de proba\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Rede segura")  .center_screen();  draw_gallery_gl();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "he" => { println!("\nרשת בטוחה - פטיש\n\nפָּשׁוּט ממשק משתמש גרפי בשביל ה רשת בטוחה\n\n# בקשות ל רשת בטוחה נעשים באמצעות CLI בטוח\n#\n# הורדות מוצלחות מוכנסות לתיקייה ./הורדות/\n# יחד עם יומנים וכל שגיאה מהרשת.\n#\n# אם ה- CLI בטוח אינו זמין, ייתכן שיופיעו כאן שגיאות.\n\n--------------\nרשת המבחנים\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("רשת בטוחה")  .center_screen();  draw_gallery_he();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "hi" => { println!("\nसुरक्षित नेटवर्क - हथौड़ा\n\nसरल ग्राफिकल यूज़र इंटरफ़ेस के लिए सुरक्षित नेटवर्क\n\n# के लिए अनुरोध सुरक्षित नेटवर्क के माध्यम से किया जाता है सुरक्षित सीएलआई\n#\n# सफल डाउनलोड फ़ोल्डर में डाल दिए जाते हैं ./डाउनलोड/\n# लॉग और नेटवर्क से किसी भी त्रुटि के साथ।\n#\n# यदि सुरक्षित सीएलआई उपलब्ध नहीं है, तो त्रुटियां यहां दिखाई दे सकती हैं।\n\n--------------\nटेस्ट नेटवर्क\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("सुरक्षित नेटवर्क")  .center_screen();  draw_gallery_hi();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "hr" => { println!("\nSigurna mreža - čekić\n\nJednostavan Grafičko korisničko sučelje za Sigurna mreža\n\n# Zahtjevi za Sigurna mreža izrađuju se putem Siguran CLI\n#\n# Uspješna preuzimanja stavljaju se u mapu ./Preuzimanja/\n# zajedno s zapisnicima i svim pogreškama s mreže.\n#\n# Ako Sigurni CLI nije dostupan, ovdje se mogu prikazati pogreške.\n\n--------------\ntestna mreža\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Sigurna mreža")  .center_screen();  draw_gallery_hr();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ht" => { println!("\nRezo ki an sekirite - mato\n\nSenp Entèfas grafik itilizatè pou Rezo ki an sekirite\n\n# Demann nan Rezo ki an sekirite yo te fè via la Klè san danje\n#\n# Downloads siksè yo mete nan katab la ./Downloads/\n# ansanm ak mòso bwa ak nenpòt ki erè nan rezo a.\n#\n# Si klik san danje a pa disponib, erè ka montre isit la.\n\n--------------\nrezo tès la\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Rezo ki an sekirite")  .center_screen();  draw_gallery_ht();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "hu" => { println!("\nBiztonságos hálózat - kalapács\n\nEgyszerű Grafikus felhasználói felület a Biztonságos hálózat\n\n# Kérések a Biztonságos hálózat keresztül készülnek Biztonságos CLI\n#\n# A sikeres letöltések a mappába kerülnek ./Letöltések/\n# a naplókkal és a hálózati hibákkal együtt.\n#\n# Ha a biztonságos CLI nem áll rendelkezésre, akkor itt hibák jelenhetnek meg.\n\n--------------\nteszt hálózat\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Biztonságos hálózat")  .center_screen();  draw_gallery_hu();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "hy" => { println!("\nԱնվտանգ ցանց - մուրճ\n\nՊարզ Գրաֆիկական ինտերֆեյս համար Անվտանգ ցանց\n\n# Հարցումներ դեպի Անվտանգ ցանց պատրաստվում են միջոցով Անվտանգ CLI\n#\n# Հաջող ներլցումները տեղադրվում են թղթապանակում ./Ներլցումներ/\n# տեղեկամատյանների և ցանցի ցանկացած սխալների հետ միասին:\n#\n# Եթե Safe CLI- ն մատչելի չէ, սխալները կարող են ցույց տալ այստեղ:\n\n--------------\nփորձարկման ցանց\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Անվտանգ ցանց")  .center_screen();  draw_gallery_hy();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "id" => { println!("\nJaringan Aman - Palu\n\nSederhana Antarmuka Pengguna Grafis untuk Jaringan Aman\n\n# Permintaan ke Jaringan Aman dibuat melalui CLI aman\n#\n# Unduhan yang berhasil dimasukkan ke dalam folder ./Unduhan/\n# bersama dengan log dan kesalahan apa pun dari jaringan.\n#\n# Jika CLI Aman tidak tersedia, kesalahan mungkin muncul di sini.\n\n--------------\njaringan uji\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Jaringan Aman")  .center_screen();  draw_gallery_id();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "it" => { println!("\nRete sicura - martello\n\nSemplice Interfaccia grafica utente per il Rete sicura\n\n# Richieste al Rete sicura sono realizzati tramite il CLI Sicuro\n#\n# I download riusciti vengono inseriti nella cartella ./Download/\n# insieme ai log e ad eventuali errori dalla rete.\n#\n# Se Safe CLI non è disponibile, qui potrebbero essere visualizzati degli errori.\n\n--------------\nrete di prova\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Rete sicura")  .center_screen();  draw_gallery_it();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ja" => { println!("\n安全な通信網 - ハンマー\n\n単純 グラフィカル・ユーザー・インターフェース のために 安全な通信網\n\n# へのリクエスト 安全な通信網 経由で作られています 安全なCLI\n#\n# 成功したダウンロードはフォルダに入れられます ./ダウンロード/\n# ログおよびネットワークからのエラーとともに。\n#\n# Safe CLIが使用できない場合は、ここにエラーが表示されることがあります。\n\n--------------\nテストネットワーク\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("安全な通信網")  .center_screen();  draw_gallery_ja();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "kn" => { println!("\nಸುರಕ್ಷಿತ ನೆಟ್ವರ್ಕ್ - ಸುತ್ತಿಗೆ\n\nಸರಳ ಚಿತ್ರಾತ್ಮಕ ಬಳಕೆದಾರ ಇಂಟರ್ಫೇಸ್ ಗಾಗಿ ಸುರಕ್ಷಿತ ನೆಟ್ವರ್ಕ್\n\n# ಗೆ ವಿನಂತಿಗಳು ಸುರಕ್ಷಿತ ನೆಟ್ವರ್ಕ್ ಮೂಲಕ ತಯಾರಿಸಲಾಗುತ್ತದೆ ಸುರಕ್ಷಿತ CLI\n#\n# ಯಶಸ್ವಿ ಡೌನ್‌ಲೋಡ್‌ಗಳನ್ನು ಫೋಲ್ಡರ್‌ಗೆ ಹಾಕಲಾಗಿದೆ ./ಡೌನ್ಲೋಡ್ಗಳು/\n# ಲಾಗ್‌ಗಳು ಮತ್ತು ನೆಟ್‌ವರ್ಕ್‌ನಿಂದ ಯಾವುದೇ ದೋಷಗಳು.\n#\n# ಸುರಕ್ಷಿತ CLI ಲಭ್ಯವಿಲ್ಲದಿದ್ದರೆ, ದೋಷಗಳನ್ನು ಇಲ್ಲಿ ತೋರಿಸಬಹುದು.\n\n--------------\nಪರೀಕ್ಷಾ ಜಾಲ\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("ಸುರಕ್ಷಿತ ನೆಟ್ವರ್ಕ್")  .center_screen();  draw_gallery_kn();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ko" => { println!("\n안전한 네트워크 - 망치\n\n단순한 그래픽 사용자 인터페이스 위해 안전한 네트워크\n\n# 요청 안전한 네트워크 통해 만들어집니다 안전한 CLI\n#\n# 성공적인 다운로드는 폴더에 저장됩니다 ./다운로드/\n# 로그 및 네트워크의 모든 오류와 함께.\n#\n# Safe CLI를 사용할 수 없는 경우 여기에 오류가 표시될 수 있습니다.\n\n--------------\n테스트 네트워크\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("안전한 네트워크")  .center_screen();  draw_gallery_ko();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ku" => { println!("\nTora Ewle - çakûç\n\nAsan Navrûya Bikarhênerê Grafîkî bo Tora Ewle\n\n# Daxwazan ji Tora Ewle bi navgîniyê têne çêkirin CLI ewle\n#\n# Dakêşanên serfiraz dikevin peldankê ./Daxistin/\n# digel têketin û her xeletiyên ji torê.\n#\n# Ger CLI -ya Ewle ne berdest e, dibe ku xeletî li vir xuya bikin.\n\n--------------\ntest network\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Tora Ewle")  .center_screen();  draw_gallery_ku();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "lt" => { println!("\nSaugus tinklas - plaktukas\n\nPaprasta Grafinė vartotojo sąsaja už Saugus tinklas\n\n# Prašymai į Saugus tinklas yra pagaminti per Saugi CLI\n#\n# Sėkmingi atsisiuntimai dedami į aplanką ./Atsisiuntimai/\n# kartu su žurnalais ir bet kokiomis tinklo klaidomis.\n#\n# Jei saugaus CLI nėra, čia gali būti rodomos klaidos.\n\n--------------\nbandomasis tinklas\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Saugus tinklas")  .center_screen();  draw_gallery_lt();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ml" => { println!("\nസുരക്ഷിത നെറ്റ്‌വർക്ക് - ചുറ്റിക\n\nലളിത ഗ്രാഫിക്കൽ യൂസർ ഇന്റർഫേസ് വേണ്ടി സുരക്ഷിത നെറ്റ്‌വർക്ക്\n\n# യോട് അഭ്യർത്ഥിക്കുന്നു സുരക്ഷിത നെറ്റ്‌വർക്ക് വഴിയാണ് നിർമ്മിച്ചിരിക്കുന്നത് സുരക്ഷിത CLI\n#\n# വിജയകരമായ ഡൗൺലോഡുകൾ ഫോൾഡറിൽ ഇടുന്നു ./ഡൗൺലോഡുകൾ/\n# ലോഗുകളും നെറ്റ്‌വർക്കിൽ നിന്നുള്ള ഏതെങ്കിലും പിശകുകളും സഹിതം.\n#\n# സുരക്ഷിത CLI ലഭ്യമല്ലെങ്കിൽ, പിശകുകൾ ഇവിടെ കാണിച്ചേക്കാം.\n\n--------------\nടെസ്റ്റ് നെറ്റ്‌വർക്ക്\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("സുരക്ഷിത നെറ്റ്‌വർക്ക്")  .center_screen();  draw_gallery_ml();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "mr" => { println!("\nसुरक्षित नेटवर्क - हातोडा\n\nसोपे ग्राफिकल यूजर इंटरफेस साठी सुरक्षित नेटवर्क\n\n# यांना विनंती सुरक्षित नेटवर्क द्वारे तयार केले जातात सुरक्षित CLI\n#\n# यशस्वी डाउनलोड फोल्डरमध्ये टाकले जातात ./डाउनलोड/\n# लॉगसह आणि नेटवर्कमधील कोणत्याही त्रुटींसह.\n#\n# सुरक्षित CLI उपलब्ध नसल्यास, त्रुटी येथे दिसू शकतात.\n\n--------------\nचाचणी नेटवर्क\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("सुरक्षित नेटवर्क")  .center_screen();  draw_gallery_mr();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ms" => { println!("\nRangkaian Selamat - tukul\n\nRingkas Antara Muka Pengguna Grafik untuk Rangkaian Selamat\n\n# Permintaan kepada Rangkaian Selamat dibuat melalui CLI selamat\n#\n# Muat turun yang berjaya dimasukkan ke dalam folder ./Muat turun/\n# bersama dengan log dan sebarang kesalahan dari rangkaian.\n#\n# Sekiranya CLI Selamat tidak tersedia, kesalahan mungkin ditunjukkan di sini.\n\n--------------\nrangkaian ujian\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Rangkaian Selamat")  .center_screen();  draw_gallery_ms();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "nl" => { println!("\nVeilig netwerk - hamer\n\nEenvoudig Grafische gebruikersinterface voor de Veilig netwerk\n\n# Verzoeken aan de Veilig netwerk zijn gemaakt via de Veilige CLI\n#\n# Succesvolle downloads worden in de map geplaatst ./Downloads/\n# samen met logboeken en eventuele fouten van het netwerk.\n#\n# Als de Safe CLI niet beschikbaar is, kunnen hier fouten worden weergegeven.\n\n--------------\ntestnetwerk\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Veilig netwerk")  .center_screen();  draw_gallery_nl();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "no" => { println!("\nTrygt nettverk - hammer\n\nEnkel Grafisk brukergrensesnitt for Trygt nettverk\n\n# Forespørsler til Trygt nettverk er laget via Trygt CLI\n#\n# Vellykkede nedlastinger legges inn i mappen ./Nedlastinger/\n# sammen med logger og eventuelle feil fra nettverket.\n#\n# Hvis Safe CLI ikke er tilgjengelig, kan det vises feil her.\n\n--------------\ntestnettverk\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Trygt nettverk")  .center_screen();  draw_gallery_no();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "pl" => { println!("\nBezpieczna sieć - młotek\n\nProsty Graficzny interfejs użytkownika dla Bezpieczna sieć\n\n# Prośby do Bezpieczna sieć są dokonywane za pośrednictwem Bezpieczny CLI\n#\n# Pomyślne pobrania są umieszczane w folderze ./Pliki do pobrania/\n# wraz z logami i wszelkimi błędami z sieci.\n#\n# Jeśli Safe CLI nie jest dostępny, w tym miejscu mogą pojawić się błędy.\n\n--------------\nsieć testowa\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Bezpieczna sieć")  .center_screen();  draw_gallery_pl();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "pt" => { println!("\nRede Segura - martelo\n\nSimples Interface gráfica do usuário para o Rede Segura\n\n# Pedidos para o Rede Segura são feitos através do CLI seguro\n#\n# Downloads bem-sucedidos são colocados na pasta ./Transferências/\n# junto com os registros e quaisquer erros da rede.\n#\n# Se o Safe CLI não estiver disponível, podem aparecer erros aqui.\n\n--------------\nrede de teste\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Rede Segura")  .center_screen();  draw_gallery_pt();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ro" => { println!("\nRețea sigură - ciocan\n\nSimplu Interfață grafică pentru utilizator pentru Rețea sigură\n\n# Solicitări către Rețea sigură sunt realizate prin intermediul CLI sigur\n#\n# Descărcările de succes sunt introduse în dosar ./Descărcări/\n# împreună cu jurnalele și eventualele erori din rețea.\n#\n# Dacă CLI sigur nu este disponibil, pot apărea erori aici.\n\n--------------\ntestează rețeaua\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Rețea sigură")  .center_screen();  draw_gallery_ro();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ru" => { println!("\nБезопасная сеть - молоток\n\nПростой Графический интерфейс пользователя для Безопасная сеть\n\n# Запросы к Безопасная сеть сделаны через Безопасный интерфейс командной строки\n#\n# Успешные загрузки помещаются в папку ./Загрузки/\n# вместе с журналами и любыми ошибками из сети.\n#\n# Если Safe CLI недоступен, здесь могут отображаться ошибки.\n\n--------------\nтестовая сеть\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Безопасная сеть")  .center_screen();  draw_gallery_ru();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "sk" => { println!("\nBezpečná sieť - kladivo\n\nJednoduché Grafické užívateľské rozhranie pre Bezpečná sieť\n\n# Žiadosti na Bezpečná sieť sú vyrobené prostredníctvom Bezpečné CLI\n#\n# Úspešné sťahovanie sa vloží do priečinka ./K stiahnutiu/\n# spolu s denníkmi a všetkými chybami zo siete.\n#\n# Ak bezpečný CLI nie je k dispozícii, môžu sa tu zobraziť chyby.\n\n--------------\ntestovacia sieť\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Bezpečná sieť")  .center_screen();  draw_gallery_sk();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "sr" => { println!("\nСигурна мрежа - чекић\n\nЈедноставно Графички кориснички интерфејс за Сигурна мрежа\n\n# Захтеви за Сигурна мрежа израђују се путем Сафе ЦЛИ\n#\n# Успешна преузимања се стављају у фасциклу ./Преузимања/\n# заједно са евиденцијама и евентуалним грешкама са мреже.\n#\n# Ако Сафе ЦЛИ није доступан, грешке се могу приказати овде.\n\n--------------\nтест нетворк\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Сигурна мрежа")  .center_screen();  draw_gallery_sr();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "sv" => { println!("\nSäkert nätverk - hammare\n\nEnkel Grafiskt användargränssnitt för Säkert nätverk\n\n# Förfrågningar till Säkert nätverk görs via Säkert CLI\n#\n# Lyckade nedladdningar läggs i mappen ./Nedladdningar/\n# tillsammans med loggar och eventuella fel från nätverket.\n#\n# Om Safe CLI inte är tillgängligt kan fel visas här.\n\n--------------\ntestnätverk\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Säkert nätverk")  .center_screen();  draw_gallery_sv();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "sw" => { println!("\nMtandao salama - hammare\n\nEnkel Grafiskt användargränssnitt kwa Mtandao salama\n\n# Förfrågningar till Mtandao salama görs via Säkert CLI\n#\n# Lyckade nedladdningar läggs i mappen ./Vipakuzi/\n# pamoja na magogo na makosa yoyote kutoka kwa mtandao.\n#\n# Ikiwa CLI Salama haipatikani, makosa yanaweza kuonyesha hapa.\n\n--------------\nmtandao wa mtihani\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Mtandao salama")  .center_screen();  draw_gallery_sw();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ta" => { println!("\nபாதுகாப்பான நெட்வொர்க் - சுத்தி\n\nஎளிய வரைகலை பயனாளர் இடைமுகம் அதற்காக பாதுகாப்பான நெட்வொர்க்\n\n# க்கான கோரிக்கைகள் பாதுகாப்பான நெட்வொர்க் மூலம் செய்யப்படுகின்றன பாதுகாப்பான CLI\n#\n# வெற்றிகரமான பதிவிறக்கங்கள் கோப்புறையில் வைக்கப்படுகின்றன ./பதிவிறக்கங்கள்/\n# பதிவுகள் மற்றும் நெட்வொர்க்கிலிருந்து ஏதேனும் பிழைகள்.\n#\n# பாதுகாப்பான CLI கிடைக்கவில்லை என்றால், பிழைகள் இங்கே காட்டப்படலாம்.\n\n--------------\nடெஸ்ட் நெட்வொர்க்\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("பாதுகாப்பான நெட்வொர்க்")  .center_screen();  draw_gallery_ta();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "te" => { println!("\nసురక్షిత నెట్‌వర్క్ - సుత్తి\n\nసింపుల్ గ్రాఫికల్ యూజర్ ఇంటర్ఫేస్ కొరకు సురక్షిత నెట్‌వర్క్\n\n# కు అభ్యర్థనలు సురక్షిత నెట్‌వర్క్ ద్వారా తయారు చేయబడతాయి సురక్షితమైన CLI\n#\n# విజయవంతమైన డౌన్‌లోడ్‌లు ఫోల్డర్‌లో పెట్టబడ్డాయి ./డౌన్‌లోడ్‌లు/\n# లాగ్‌లతో పాటు నెట్‌వర్క్ నుండి ఏవైనా లోపాలు.\n#\n# సురక్షిత CLI అందుబాటులో లేనట్లయితే, లోపాలు ఇక్కడ చూపబడవచ్చు.\n\n--------------\nపరీక్ష నెట్‌వర్క్\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("సురక్షిత నెట్‌వర్క్")  .center_screen();  draw_gallery_te();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "th" => { println!("\nเครือข่ายที่ปลอดภัย - ค้อน\n\nเรียบง่าย ส่วนต่อประสานกราฟิกกับผู้ใช้ สำหรับ เครือข่ายที่ปลอดภัย\n\n# การร้องขอไปยัง เครือข่ายที่ปลอดภัย จะทำผ่าน คลีนิกที่ปลอดภัย\n#\n# การดาวน์โหลดที่สำเร็จจะถูกใส่ลงในโฟลเดอร์ ./ดาวน์โหลด/\n# พร้อมกับบันทึกและข้อผิดพลาดจากเครือข่าย\n#\n# หากไม่มี Safe CLI ข้อผิดพลาดอาจแสดงที่นี่\n\n--------------\nทดสอบเครือข่าย\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("เครือข่ายที่ปลอดภัย")  .center_screen();  draw_gallery_th();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "tk" => { println!("\nYgtybarly ulgam - çekiç\n\nBasit Grafiksel kullanıcı arayüzü üçin Ygtybarly ulgam\n\n# Talepler Ygtybarly ulgam aracılığıyla yapılır Güvenli CLI\n#\n# Başarılı indirmeler klasöre konur ./Üklemeler/\n# surnallar we tordaky islendik ýalňyşlyklar bilen bilelikde.\n#\n# Ygtybarly CLI elýeterli däl bolsa, bu ýerde ýalňyşlyklar görkezilip bilner.\n\n--------------\nsynag tory\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Ygtybarly ulgam")  .center_screen();  draw_gallery_tk();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "tr" => { println!("\nGüvenli Ağ - çekiç\n\nBasit Grafiksel kullanıcı arayüzü için Güvenli Ağ\n\n# Talepler Güvenli Ağ aracılığıyla yapılır Güvenli CLI\n#\n# Başarılı indirmeler klasöre konur ./İndirilenler/\n# günlükler ve ağdaki hatalarla birlikte.\n#\n# Güvenli CLI mevcut değilse, burada hatalar görünebilir.\n\n--------------\ntest ağı\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Güvenli Ağ")  .center_screen();  draw_gallery_tr();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "tt" => { println!("\nКуркынычсыз челтәр - чүкеч\n\nГади График кулланучы интерфейсы өчен Куркынычсыз челтәр\n\n# .Әр сүзнең Куркынычсыз челтәр аша ясала Куркынычсыз CLI\n#\n# Уңышлы йөкләүләр папкага кертелә ./Йөкләүләр/\n# бүрәнәләр һәм челтәрдәге хаталар белән бергә.\n#\n# Әгәр дә куркынычсыз CLI мөмкин булмаса, монда хаталар күрсәтелергә мөмкин.\n\n--------------\nтест челтәре\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Куркынычсыз челтәр")  .center_screen();  draw_gallery_tt();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ug" => { println!("\nبىخەتەر تور - بولقا\n\nئاددىي گرافىكلىق ئىشلەتكۈچى ئارايۈزى ئۈچۈن بىخەتەر تور\n\n# تەلەپلەر بىخەتەر تور ئارقىلىق ياسالغان بىخەتەر CLI\n#\n# مۇۋەپپەقىيەتلىك چۈشۈرۈش ھۆججەت قىسقۇچقا سېلىنىدۇ ./چۈشۈرۈش/\n# خاتىرە ۋە توردىكى خاتالىقلار بىلەن بىللە.\n#\n# ئەگەر بىخەتەر CLI نى ئىشلەتكىلى بولمىسا ، بۇ يەردە خاتالىق كۆرۈلۈشى مۇمكىن.\n\n--------------\nسىناق تورى\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("بىخەتەر تور")  .center_screen();  draw_gallery_ug();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "uk" => { println!("\nБезпечна мережа - молоток\n\nПростий Графічний інтерфейс користувача для Безпечна мережа\n\n# Запити до Безпечна мережа здійснюються через Безпечний CLI\n#\n# Успішні завантаження зберігаються в папці ./Завантаження/\n# разом з журналами та будь -якими помилками з мережі.\n#\n# Якщо безпечний CLI недоступний, тут можуть відображатися помилки.\n\n--------------\nтестова мережа\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Безпечна мережа")  .center_screen();  draw_gallery_uk();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "ur" => { println!("\nمحفوظ نیٹ ورک۔ - ہتھوڑا\n\nسادہ۔ گرافیکل یوزر انٹرفیس۔ کے لئے محفوظ نیٹ ورک۔\n\n# سے درخواستیں۔ محفوظ نیٹ ورک۔ کے ذریعے بنائے جاتے ہیں۔ محفوظ سی ایل آئی۔\n#\n# کامیاب ڈاؤن لوڈز فولڈر میں ڈال دیے جاتے ہیں۔ ./ڈاؤن لوڈ/\n# لاگز اور نیٹ ورک کی کسی بھی غلطی کے ساتھ۔\n#\n# اگر محفوظ CLI دستیاب نہیں ہے تو ، غلطیاں یہاں ظاہر ہو سکتی ہیں۔\n\n--------------\nٹیسٹ نیٹ ورک\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("محفوظ نیٹ ورک۔")  .center_screen();  draw_gallery_ur();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "vi" => { println!("\nMạng an toàn - cây búa\n\nĐơn giản Giao diện đồ họa người dùng cho Mạng an toàn\n\n# Yêu cầu đối với Mạng an toàn được thực hiện thông qua CLI an toàn\n#\n# Tải xuống thành công được đưa vào thư mục ./Tải xuống/\n# cùng với nhật ký và bất kỳ lỗi nào từ mạng.\n#\n# Nếu CLI An toàn không khả dụng, các lỗi có thể hiển thị ở đây.\n\n--------------\nmạng thử nghiệm\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Mạng an toàn")  .center_screen();  draw_gallery_vi();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "xh" => { println!("\nInethiwekhi Ekhuselekileyo - isando\n\nElula Ujongano lomsebenzisi womzobo kuba Inethiwekhi Ekhuselekileyo\n\n# Izicelo eziya kwifayile ye- Inethiwekhi Ekhuselekileyo zenziwe nge I-CLI ekhuselekileyo\n#\n# Ukhuphelo oluyimpumelelo lubekwa kwifolda ./Ukhuphelo/\n# kunye neengodo kunye naziphi na iimpazamo ezivela kwinethiwekhi.\n#\n# Ukuba i-CLI ekhuselekileyo ayinakufunyanwa, iimpazamo zingabonisa apha.\n\n--------------\ninethiwekhi yovavanyo\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Inethiwekhi Ekhuselekileyo")  .center_screen();  draw_gallery_xh();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "zh_traditional" => { println!("\n安全網絡 - 錘子\n\n簡單的 圖形用戶界面 為了 安全網絡\n\n# 向 安全網絡 是通過 安全的命令行界面\n#\n# 下載成功放入文件夾 ./下載/\n# 以及來自網絡的日誌和任何錯誤。\n#\n# 如果 Safe CLI 不可用，則此處可能會顯示錯誤。\n\n--------------\n測試網\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("安全網絡")  .center_screen();  draw_gallery_zh_traditional();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 "zh" => { println!("\n安全网络 - 锤子\n\n简单的 图形用户界面 为了 安全网络\n\n# 向 安全网络 是通过 安全的命令行界面\n#\n# 下载成功放入文件夹 ./下载/\n# 以及来自网络的日志和任何错误。\n#\n# 如果 Safe CLI 不可用，则此处可能会显示错误。\n\n--------------\n测试网\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("安全网络")  .center_screen();  draw_gallery_zh();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }
 
// ## en + default ##
 _ => { println!("\nSafe Network - hammer\n\nSimple Graphical User Interface for the Safe Network\n\n# Requests to the Safe Network are made via the Safe CLI\n#\n# Successful downloads are put into the folder ./Downloads/\n# along with logs and any errors from the network.\n#\n# If the Safe CLI is not avaliable, errors may show here.\n\n--------------\nTest Network\n--------------\n");  let app = app::App::default().with_scheme(app::Scheme::Gtk);  app::background(255, 255, 255);  let mut wind = Window::default()  .with_size(800, 600)  .with_label("Safe Network")  .center_screen();  draw_gallery();  wind.make_resizable(true);  wind.end();  wind.show(); match app.run() { Ok(_) => {}, Err(err) => { println!("Error from fltk app!\n{}", err); } } }

// ## /en + default ##
} // ## /match lang ##

} // ## /main ##



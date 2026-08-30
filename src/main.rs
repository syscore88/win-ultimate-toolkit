#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::os::windows::process::CommandExt;
use std::fs;
use std::path::PathBuf;

const CREATE_NO_WINDOW: u32 = 0x08000000;

const VCREDIST_META_ID: &str = "Meta.VCRedistAll";
const VCREDIST_IDS: [&str; 12] = [
    "Microsoft.VCRedist.2005.x64",
    "Microsoft.VCRedist.2005.x86",
    "Microsoft.VCRedist.2008.x64",
    "Microsoft.VCRedist.2008.x86",
    "Microsoft.VCRedist.2010.x64",
    "Microsoft.VCRedist.2010.x86",
    "Microsoft.VCRedist.2012.x64",
    "Microsoft.VCRedist.2012.x86",
    "Microsoft.VCRedist.2013.x64",
    "Microsoft.VCRedist.2013.x86",
    "Microsoft.VCRedist.2015+.x64",
    "Microsoft.VCRedist.2015+.x86",
];

fn program_info(winget_id: &str, lang_is_pl: bool) -> &'static str {
    if lang_is_pl {
        match winget_id {
        "NoWinget.ActivePresenter" => "Program do nagrywania ekranu i tworzenia kursów e-learningowych.",
        "axpnet.AeroFTP" => "Klient FTP/SFTP do przesyłania plików na serwery.",
        "FinalWire.AIDA64.Extreme" => "Zaawansowane narzędzie do diagnostyki i informacji o podzespołach komputera.",
        "NoWinget.AnimatedWallpaperMaker" => "Tworzy animowane tapety pulpitu.",
        "AnyDeskSoftwareGmbH.AnyDesk" => "Program do zdalnego pulpitu i zdalnej pomocy technicznej.",
        "AOMEI.Backupper.Standard" => "Tworzenie kopii zapasowych systemu, dysków i plików.",
        "AOMEI.PartitionAssistant" => "Zarządzanie i partycjonowanie dysków twardych.",
        "Anysphere.Cursor" => "Edytor kodu z wbudowanym agentem AI wspomagającym programowanie.",
        "Apple.AppleMusic" => "Aplikacja do strumieniowania muzyki Apple Music.",
        "Apple.AppleTV" => "Aplikacja do oglądania filmów i seriali Apple TV+.",
        "Audacity.Audacity" => "Darmowy edytor i rejestrator dźwięku.",
        "NoWinget.Automize" => "Automatyzacja instalacji i aktualizacji programów.",
        "Vector35.BinaryNinja" => "Interaktywna platforma do inżynierii wstecznej (deasembler).",
        "Microsoft.BingWallpaper" => "Codziennie zmienia tapetę pulpitu na zdjęcia z Bing.",
        "Bitdefender.AntivirusFree" => "Darmowy program antywirusowy.",
        "BlackmagicDesign.DaVinciResolve" => "Profesjonalny montaż wideo, kolorystyka i produkcja filmowa.",
        "BleachBit.BleachBit" => "Czyści niepotrzebne pliki i zwalnia miejsce na dysku.",
        "BlenderFoundation.Blender" => "Darmowy program do grafiki i animacji 3D.",
        "Blix.BlueMail" => "Klient pocztowy obsługujący wiele kont e-mail.",
        "Brave.Brave" => "Przeglądarka internetowa skupiona na prywatności.",
        "TechSmith.Camtasia" => "Nagrywanie ekranu i montaż filmów instruktażowych.",
        "Canva.Canva" => "Narzędzie do projektowania grafik online.",
        "OpenAI.ChatGPT" => "Aplikacja klienta czatu z asystentem AI ChatGPT.",
        "Clonezilla.Clonezilla" => "Narzędzie do klonowania i tworzenia obrazów dysków.",
        "Codex.Codex" => "Narzędzie AI od OpenAI do wspomagania programowania.",
        "DeepL.DeepL" => "Tłumacz tekstu wykorzystujący sztuczną inteligencję.",
        "Microsoft.DirectX" => "Zestaw komponentów multimedialnych wymaganych przez gry i aplikacje.",
        "Discord.Discord" => "Komunikator głosowy i tekstowy dla społeczności i graczy.",
        "Disney.DisneyPlus" => "Serwis streamingowy Disney+.",
        "DMDE.DMDE" => "Narzędzie do odzyskiwania utraconych danych z dysków.",
        "Docker.DockerDesktop" => "Platforma do uruchamiania aplikacji w kontenerach.",
        "DOSBox.DOSBox" => "Emulator systemu DOS do uruchamiania starych gier i programów.",
        "Doublecmd.Doublecmd" => "Dwupanelowy menedżer plików.",
        "NoWinget.Doxillion" => "Konwerter formatów dokumentów.",
        "Dropbox.Dropbox" => "Przechowywanie i synchronizacja plików w chmurze.",
        "Duolingo.Duolingo" => "Aplikacja do nauki języków obcych.",
        "EaseUS.PartitionMaster" => "Zarządzanie partycjami dysku twardego.",
        "EaseUS.TodoBackup" => "Tworzenie kopii zapasowych systemu i plików.",
        "NoWinget.FileConverter" => "Konwertuje pliki bezpośrednio z menu kontekstowego Eksploratora.",
        "TimKosse.FileZilla.Client" => "Klient FTP/SFTP do przesyłania plików.",
        "FireAlpaca.FireAlpaca" => "Darmowy program do rysowania cyfrowego.",
        "Mozilla.Firefox" => "Przeglądarka internetowa Mozilla Firefox.",
        "Image-Line.FLStudio" => "Program do produkcji muzycznej (DAW).",
        "NoWinget.FormatFactory" => "Konwerter plików wideo, audio i obrazów.",
        "NoWinget.freac" => "Konwerter i ripper plików audio.",
        "FreeCAD.FreeCAD" => "Darmowy program do projektowania inżynierskiego 3D (CAD).",
        "FreeDownloadManager.FreeDownloadManager" => "Menedżer pobierania plików przyśpieszający ściąganie.",
        "GIMP.GIMP.3" => "Darmowy zaawansowany edytor grafiki rastrowej.",
        "Git.Git" => "System kontroli wersji dla programistów.",
        "Google.Chrome" => "Przeglądarka internetowa Google Chrome.",
        "Google.GoogleDrive" => "Przechowywanie i synchronizacja plików w chmurze Google.",
        "Google.PlatformTools" => "Narzędzia ADB/Fastboot do pracy z urządzeniami Android.",
        "GParted.GParted" => "Graficzne narzędzie do partycjonowania dysków.",
        "HandBrake.HandBrake" => "Konwerter i kompresor plików wideo.",
        "Hetman.Recovery" => "Odzyskiwanie skasowanych plików i danych.",
        "CPUID.HWMonitor" => "Monitoruje temperatury i napięcia podzespołów komputera.",
        "Inkscape.Inkscape" => "Darmowy edytor grafiki wektorowej.",
        "Instagram.Instagram" => "Aplikacja społecznościowa do udostępniania zdjęć i filmów.",
        "IrfanSkiljan.IrfanView" => "Szybka przeglądarka i konwerter obrazów.",
        "AppWork.JDownloader" => "Menedżer do masowego pobierania plików z internetu.",
        "KDE.Kate" => "Zaawansowany edytor tekstu dla programistów.",
        "KDE.Kdenlive" => "Darmowy nieliniowy edytor wideo.",
        "DominikReichl.KeePass" => "Menedżer haseł przechowujący dane w zaszyfrowanej bazie.",
        "KiCad.KiCad" => "Program do projektowania płytek elektronicznych PCB.",
        "KDE.Krita" => "Darmowy program do malowania i rysowania cyfrowego.",
        "TheDocumentFoundation.LibreOffice" => "Darmowy pakiet biurowy (edytor tekstu, arkusze, prezentacje).",
        "LMMS.LMMS" => "Darmowy program do tworzenia muzyki (DAW).",
        "LMStudio.LMStudio" => "Uruchamianie i testowanie modeli AI (LLM) lokalnie na komputerze.",
        "Mailbird.Mailbird" => "Klient pocztowy do zarządzania wieloma skrzynkami e-mail.",
        "MediaHuman.AudioConverter" => "Konwerter formatów plików audio.",
        "Mega.MEGASync" => "Przechowywanie i synchronizacja plików w chmurze MEGA.",
        "MEGOGO.MEGOGO" => "Serwis streamingowy z filmami i serialami.",
        "Microsoft.PCManager" => "Oficjalne narzędzie Microsoft do optymalizacji i czyszczenia systemu.",
        "Microsoft.PowerToys" => "Zestaw zaawansowanych narzędzi zwiększających produktywność w Windows.",
        "Microsoft.VisualStudio.2022.Community" => "Zintegrowane środowisko programistyczne (IDE) Microsoft.",
        "GNU.MidnightCommander" => "Dwupanelowy menedżer plików w stylu tekstowym.",
        "Mixxx.Mixxx" => "Darmowy program do miksowania muzyki dla DJ-ów.",
        "MartinPesch.mp3DirectCut" => "Bezstratne przycinanie i edycja plików MP3.",
        "StevenMayall.MusicBee" => "Odtwarzacz muzyki i menedżer biblioteki audio.",
        "Netflix.Netflix" => "Serwis streamingowy Netflix z filmami i serialami.",
        "Nextcloud.NextcloudDesktop" => "Klient synchronizujący pliki z prywatną chmurą Nextcloud.",
        "Notepad++.Notepad++" => "Zaawansowany edytor tekstu i kodu źródłowego.",
        "Nvidia.GeForceNow" => "Usługa chmurowego grania NVIDIA GeForce NOW.",
        "OBSProject.OBSStudio" => "Darmowy program do nagrywania ekranu i streamingu na żywo.",
        "OCBASE.OCCT" => "Narzędzie do testów stabilności i obciążenia podzespołów PC.",
        "KDE.Okular" => "Przeglądarka dokumentów PDF i innych formatów.",
        "Ollama.Ollama" => "Uruchamianie i zarządzanie lokalnymi modelami AI (LLM) na komputerze.",
        "ONLYOFFICE.DesktopEditors" => "Pakiet biurowy do edycji dokumentów, arkuszy i prezentacji.",
        "OpenClaw.OpenClawCompanion" => "Aplikacja towarzysząca dla asystenta AI OpenClaw (zasobnik systemowy, integracje).",
        "SST.opencode" => "Narzędzie AI wspomagające programowanie w terminalu.",
        "Opera.Opera" => "Przeglądarka internetowa Opera.",
        "Opera.OperaGX" => "Przeglądarka internetowa Opera GX dla graczy.",
        "dotPDN.PaintDotNet" => "Prosty i lekki edytor grafiki rastrowej.",
        "Perplexity.Perplexity" => "Aplikacja wyszukiwarki i asystenta AI Perplexity.",
        "Picsart.Picsart" => "Edytor zdjęć i grafiki z funkcjami AI.",
        "Pinterest.Pinterest" => "Aplikacja społecznościowa do odkrywania inspiracji wizualnych.",
        "RedHat.Podman-Desktop" => "Graficzny interfejs do zarządzania kontenerami Podman.",
        "Daum.PotPlayer" => "Wszechstronny odtwarzacz multimedialny.",
        "Amazon.PrimeVideo" => "Serwis streamingowy Amazon Prime Video.",
        "ProtonTechnologies.ProtonVPN" => "Aplikacja VPN chroniąca prywatność w sieci.",
        "Qalculate.Qalculate" => "Rozbudowany kalkulator naukowy.",
        "qBittorrent.qBittorrent" => "Darmowy klient sieci BitTorrent.",
        "Qmmp.Qmmp" => "Lekki odtwarzacz muzyki w stylu Winampa.",
        "RawTherapee.RawTherapee" => "Program do obróbki zdjęć w formacie RAW.",
        "Cockos.REAPER" => "Profesjonalna stacja do produkcji muzycznej (DAW).",
        "reMarkable.reMarkable" => "Aplikacja do synchronizacji z tabletem reMarkable.",
        "Rescuezilla.Rescuezilla" => "Tworzenie kopii zapasowych i przywracanie obrazów dysków.",
        "VSRevoGroup.RevoUninstaller" => "Zaawansowane odinstalowywanie programów z resztkami plików.",
        "Roblox.Roblox" => "Platforma gier online Roblox.",
        "Rufus.Rufus" => "Tworzenie rozruchowych nośników USB.",
        "RustDesk.RustDesk" => "Darmowy program do zdalnego pulpitu.",
        "Meltytech.Shotcut" => "Darmowy edytor wideo.",
        "PaulPacifico.ShutterEncoder" => "Konwerter plików wideo i audio.",
        "OpenWhisperSystems.Signal" => "Szyfrowany komunikator do rozmów i wiadomości.",
        "Smarty.Uninstaller" => "Narzędzie do odinstalowywania programów.",
        "Snap.Snapchat" => "Aplikacja społecznościowa do wysyłania zdjęć i filmów.",
        "Spotify.Spotify" => "Serwis streamingowy muzyki Spotify.",
        "Valve.Steam" => "Platforma dystrybucji gier komputerowych Steam.",
        "Streamlabs.StreamlabsDesktop" => "Program do streamingu na żywo z nakładkami dla twórców.",
        "NoWinget.StrongRecovery" => "Narzędzie do odzyskiwania danych.",
        "SumatraPDF.SumatraPDF" => "Lekka i szybka przeglądarka plików PDF.",
        "Telegram.TelegramDesktop" => "Komunikator do wiadomości i rozmów Telegram.",
        "Ritlabs.TheBat" => "Klient pocztowy z zaawansowanym szyfrowaniem.",
        "Mozilla.Thunderbird" => "Darmowy klient pocztowy Mozilla Thunderbird.",
        "TIDAL.TIDAL" => "Serwis streamingowy muzyki w wysokiej jakości.",
        "TikTok.TikTok" => "Aplikacja społecznościowa z krótkimi filmami wideo.",
        "Transmission.Transmission" => "Lekki klient sieci BitTorrent.",
        "Twitch.Twitch" => "Platforma streamingowa dla graczy i twórców.",
        "Meta.VCRedistAll" => "Instaluje wszystkie wersje bibliotek Visual C++ (2005-2015+, x86 i x64) wymagane przez różne programy.",
        "ventoy.Ventoy" => "Tworzenie rozruchowych nośników USB z wieloma obrazami ISO.",
        "Oracle.VirtualBox" => "Program do wirtualizacji systemów operacyjnych.",
        "AtomixProductions.VirtualDJ" => "Program do miksowania muzyki dla DJ-ów.",
        "VideoLAN.VLC" => "Uniwersalny odtwarzacz multimedialny obsługujący niemal każdy format.",
        "VivaldiTechnologies.Vivaldi" => "Przeglądarka internetowa Vivaldi z rozbudowanymi opcjami personalizacji.",
        "VSCodium.VSCodium" => "Edytor kodu źródłowego, wolna od telemetrii wersja VS Code.",
        "WhatsApp.WhatsApp" => "Komunikator do wiadomości i rozmów WhatsApp.",
        "RamenSoftware.Windhawk" => "Modyfikuje wygląd i zachowanie Windows za pomocą wtyczek.",
        "WireGuard.WireGuard" => "Prosty, szybki i nowoczesny VPN oparty na nowoczesnej kryptografii.",
        "WiseCleaner.WiseCare365" => "Czyszczenie, optymalizacja i przyśpieszanie systemu Windows (Wise Cleaner).",
        "NoWinget.Wix" => "Kreator stron internetowych online Wix.",
        "XnSoft.XnViewMP" => "Przeglądarka i konwerter obrazów obsługująca wiele formatów.",
        "ZenBrowser.Zen" => "Przeglądarka internetowa skupiona na estetyce i produktywności.",
        "Zoom.Zoom" => "Program do wideokonferencji i spotkań online.",
        "NoWinget.4KVideoDownloader" => "Pobiera filmy, playlisty i napisy z YouTube oraz innych serwisów w wysokiej jakości.",
        "Klocman.BulkCrapUninstaller" => "Zaawansowane narzędzie do masowego odinstalowywania programów wraz z pozostałościami.",
        "TGRMNSoftware.BulkRenameUtility" => "Narzędzie do masowej zmiany nazw wielu plików jednocześnie.",
        "darktable.darktable" => "Darmowy program do obróbki zdjęć RAW i zarządzania fotografiami.",
        "NoWinget.EaseUSVideoDownloader" => "Pobiera filmy z YouTube i innych popularnych serwisów wideo.",
        "Fastfetch-cli.Fastfetch" => "Szybkie narzędzie wiersza poleceń wyświetlające informacje o systemie i sprzęcie.",
        "FreeTubeApp.FreeTube" => "Prywatna, wolna od reklam aplikacja do oglądania filmów z YouTube.",
        "GSmartControl.GSmartControl" => "Sprawdza stan zdrowia dysków twardych i SSD (SMART).",
        "ImputNet.Helium" => "Lekka, szybka przeglądarka internetowa oparta na Chromium, skupiona na prywatności.",
        "NoWinget.iTubeGo" => "Pobiera filmy i muzykę z YouTube oraz innych serwisów wideo.",
        "NoWinget.JavaOpenJDK" => "Darmowe środowisko uruchomieniowe i zestaw narzędzi do programowania w Javie (OpenJDK).",
        "NoWinget.LockHunter" => "Odblokowuje i usuwa pliki zablokowane przez inne procesy w systemie.",
        "nomacs.nomacs" => "Lekka i szybka przeglądarka obrazów z funkcjami synchronizacji i edycji.",
        "CalcProgrammer1.OpenRGB" => "Otwarte oprogramowanie do sterowania podświetleniem RGB podzespołów komputerowych.",
        "Pidgin.Pidgin" => "Uniwersalny komunikator obsługujący wiele protokołów czatu jednocześnie.",
        "PuTTY.PuTTY" => "Klient SSH, Telnet i szeregowy do zdalnego łączenia się z serwerami.",
        "Rainmeter.Rainmeter" => "Narzędzie do tworzenia i wyświetlania konfigurowalnych widżetów na pulpicie.",
        "Genymobile.scrcpy" => "Wyświetla i steruje ekranem telefonu Android z poziomu komputera.",
        "NoWinget.Sefirah" => "Łączy telefon z komputerem (powiadomienia, schowek, pliki) – alternatywa dla KDE Connect.",
        "ShareX.ShareX" => "Zaawansowane narzędzie do zrzutów ekranu, nagrywania i udostępniania plików.",
        "NoWinget.SmartRename" => "Narzędzie do masowej, inteligentnej zmiany nazw plików.",
        "NoWinget.SnappyDriverInstaller" => "Automatycznie wyszukuje i instaluje najnowsze sterowniki do podzespołów komputera.",
        "KRTirtho.Spotube" => "Odtwarzacz muzyki korzystający z Spotify i YouTube Music bez konta Premium.",
        "NoWinget.Tenacity" => "Darmowy edytor i rejestrator dźwięku, rozwijany fork Audacity.",
        "zhongyang219.TrafficMonitor" => "Monitoruje w czasie rzeczywistym prędkość sieci oraz obciążenie CPU i pamięci na pasku zadań.",
        "NoWinget.Wipe" => "Usuwa śmieci i pliki tymczasowe, czyszcząc system Windows.",
        "NoWinget.ytDownloader" => "Prosta aplikacja do pobierania filmów i muzyki z YouTube oraz innych serwisów.",
            _ => "Brak dodatkowych informacji o tym programie.",
        }
    } else {
        match winget_id {
        "NoWinget.ActivePresenter" => "Screen recorder and e-learning course authoring tool.",
        "axpnet.AeroFTP" => "FTP/SFTP client for uploading files to servers.",
        "FinalWire.AIDA64.Extreme" => "Advanced system diagnostics and hardware information tool.",
        "NoWinget.AnimatedWallpaperMaker" => "Creates animated desktop wallpapers.",
        "AnyDeskSoftwareGmbH.AnyDesk" => "Remote desktop and remote support software.",
        "AOMEI.Backupper.Standard" => "Backup software for system, disks, and files.",
        "AOMEI.PartitionAssistant" => "Disk partition management tool.",
        "Anysphere.Cursor" => "AI-powered code editor with a built-in coding agent.",
        "Apple.AppleMusic" => "Apple's music streaming app.",
        "Apple.AppleTV" => "App for streaming Apple TV+ movies and shows.",
        "Audacity.Audacity" => "Free audio editor and recorder.",
        "NoWinget.Automize" => "Automates software installation and updates.",
        "Vector35.BinaryNinja" => "Interactive reverse engineering / disassembler platform.",
        "Microsoft.BingWallpaper" => "Sets Bing's daily photo as your desktop wallpaper.",
        "Bitdefender.AntivirusFree" => "Free antivirus protection software.",
        "BlackmagicDesign.DaVinciResolve" => "Professional video editing, color grading, and post-production.",
        "BleachBit.BleachBit" => "Cleans junk files and frees up disk space.",
        "BlenderFoundation.Blender" => "Free 3D modeling and animation software.",
        "Blix.BlueMail" => "Email client supporting multiple accounts.",
        "Brave.Brave" => "Privacy-focused web browser.",
        "TechSmith.Camtasia" => "Screen recording and video editing for tutorials.",
        "Canva.Canva" => "Online graphic design tool.",
        "OpenAI.ChatGPT" => "Desktop client for OpenAI's ChatGPT assistant.",
        "Clonezilla.Clonezilla" => "Disk cloning and imaging tool.",
        "Codex.Codex" => "OpenAI's AI coding assistant tool.",
        "DeepL.DeepL" => "AI-powered text translation tool.",
        "Microsoft.DirectX" => "Multimedia runtime components required by many games and apps.",
        "Discord.Discord" => "Voice and text chat app for communities and gamers.",
        "Disney.DisneyPlus" => "Disney+ video streaming app.",
        "DMDE.DMDE" => "Data recovery tool for lost files.",
        "Docker.DockerDesktop" => "Platform for building and running containerized applications.",
        "DOSBox.DOSBox" => "DOS emulator for running old games and software.",
        "Doublecmd.Doublecmd" => "Dual-pane file manager.",
        "NoWinget.Doxillion" => "Document format converter.",
        "Dropbox.Dropbox" => "Cloud storage and file synchronization.",
        "Duolingo.Duolingo" => "Language learning app.",
        "EaseUS.PartitionMaster" => "Disk partition management software.",
        "EaseUS.TodoBackup" => "System and file backup software.",
        "NoWinget.FileConverter" => "Converts files directly from Explorer's right-click menu.",
        "TimKosse.FileZilla.Client" => "FTP/SFTP client for file transfers.",
        "FireAlpaca.FireAlpaca" => "Free digital painting software.",
        "Mozilla.Firefox" => "Mozilla Firefox web browser.",
        "Image-Line.FLStudio" => "Digital audio workstation for music production.",
        "NoWinget.FormatFactory" => "Video, audio, and image file converter.",
        "NoWinget.freac" => "Audio converter and CD ripper.",
        "FreeCAD.FreeCAD" => "Free parametric 3D CAD modeling software.",
        "FreeDownloadManager.FreeDownloadManager" => "Download manager that accelerates file downloads.",
        "GIMP.GIMP.3" => "Free advanced raster graphics editor.",
        "Git.Git" => "Version control system for developers.",
        "Google.Chrome" => "Google Chrome web browser.",
        "Google.GoogleDrive" => "Google's cloud storage and file sync app.",
        "Google.PlatformTools" => "ADB/Fastboot tools for working with Android devices.",
        "GParted.GParted" => "Graphical disk partitioning tool.",
        "HandBrake.HandBrake" => "Video transcoder and compressor.",
        "Hetman.Recovery" => "Recovers deleted files and lost data.",
        "CPUID.HWMonitor" => "Monitors PC hardware temperatures and voltages.",
        "Inkscape.Inkscape" => "Free vector graphics editor.",
        "Instagram.Instagram" => "Photo and video sharing social media app.",
        "IrfanSkiljan.IrfanView" => "Fast image viewer and converter.",
        "AppWork.JDownloader" => "Bulk download manager for internet files.",
        "KDE.Kate" => "Advanced text editor for developers.",
        "KDE.Kdenlive" => "Free non-linear video editor.",
        "DominikReichl.KeePass" => "Password manager with encrypted database storage.",
        "KiCad.KiCad" => "Electronic PCB design software.",
        "KDE.Krita" => "Free digital painting and illustration software.",
        "TheDocumentFoundation.LibreOffice" => "Free office suite (word processor, spreadsheets, presentations).",
        "LMMS.LMMS" => "Free digital audio workstation for music creation.",
        "LMStudio.LMStudio" => "Run and test local AI language models on your PC.",
        "Mailbird.Mailbird" => "Email client for managing multiple mailboxes.",
        "MediaHuman.AudioConverter" => "Audio file format converter.",
        "Mega.MEGASync" => "MEGA cloud storage and file sync client.",
        "MEGOGO.MEGOGO" => "Streaming service for movies and TV shows.",
        "Microsoft.PCManager" => "Microsoft's official PC optimization and cleanup tool.",
        "Microsoft.PowerToys" => "Set of advanced utilities to boost Windows productivity.",
        "Microsoft.VisualStudio.2022.Community" => "Microsoft's integrated development environment (IDE).",
        "GNU.MidnightCommander" => "Text-based dual-pane file manager.",
        "Mixxx.Mixxx" => "Free DJ mixing and music software.",
        "MartinPesch.mp3DirectCut" => "Lossless MP3 cutting and editing tool.",
        "StevenMayall.MusicBee" => "Music player and audio library manager.",
        "Netflix.Netflix" => "Netflix video streaming app.",
        "Nextcloud.NextcloudDesktop" => "Desktop client for syncing files with your private Nextcloud.",
        "Notepad++.Notepad++" => "Advanced text and source code editor.",
        "Nvidia.GeForceNow" => "NVIDIA GeForce NOW cloud gaming service.",
        "OBSProject.OBSStudio" => "Free screen recording and live streaming software.",
        "OCBASE.OCCT" => "PC hardware stress testing and stability tool.",
        "KDE.Okular" => "Document viewer for PDF and other formats.",
        "Ollama.Ollama" => "Run and manage local AI language models (LLM) on your PC.",
        "ONLYOFFICE.DesktopEditors" => "Office suite for documents, spreadsheets, and presentations.",
        "OpenClaw.OpenClawCompanion" => "Companion app for the OpenClaw AI assistant (system tray, integrations).",
        "SST.opencode" => "AI coding assistant tool for the terminal.",
        "Opera.Opera" => "Opera web browser.",
        "Opera.OperaGX" => "Opera GX web browser, tailored for gamers.",
        "dotPDN.PaintDotNet" => "Simple and lightweight raster image editor.",
        "Perplexity.Perplexity" => "AI-powered search and assistant app.",
        "Picsart.Picsart" => "Photo and graphic editor with AI features.",
        "Pinterest.Pinterest" => "Social app for discovering visual inspiration.",
        "RedHat.Podman-Desktop" => "Graphical UI for managing Podman containers.",
        "Daum.PotPlayer" => "Feature-rich multimedia player.",
        "Amazon.PrimeVideo" => "Amazon Prime Video streaming app.",
        "ProtonTechnologies.ProtonVPN" => "VPN app for online privacy protection.",
        "Qalculate.Qalculate" => "Powerful scientific calculator application.",
        "qBittorrent.qBittorrent" => "Free BitTorrent client.",
        "Qmmp.Qmmp" => "Lightweight Winamp-style music player.",
        "RawTherapee.RawTherapee" => "RAW photo editing and processing software.",
        "Cockos.REAPER" => "Professional digital audio workstation (DAW).",
        "reMarkable.reMarkable" => "Companion app for the reMarkable e-ink tablet.",
        "Rescuezilla.Rescuezilla" => "Disk imaging and backup/restore tool.",
        "VSRevoGroup.RevoUninstaller" => "Advanced program uninstaller that removes leftover files.",
        "Roblox.Roblox" => "Roblox online gaming platform.",
        "Rufus.Rufus" => "Creates bootable USB drives.",
        "RustDesk.RustDesk" => "Free open-source remote desktop software.",
        "Meltytech.Shotcut" => "Free open-source video editor.",
        "PaulPacifico.ShutterEncoder" => "Video and audio file converter.",
        "OpenWhisperSystems.Signal" => "Encrypted messaging and calling app.",
        "Smarty.Uninstaller" => "Program uninstaller tool.",
        "Snap.Snapchat" => "Social app for sharing photos and videos.",
        "Spotify.Spotify" => "Spotify music streaming app.",
        "Valve.Steam" => "Steam PC gaming distribution platform.",
        "Streamlabs.StreamlabsDesktop" => "Live streaming software with overlays for content creators.",
        "NoWinget.StrongRecovery" => "Data recovery utility.",
        "SumatraPDF.SumatraPDF" => "Lightweight and fast PDF viewer.",
        "Telegram.TelegramDesktop" => "Telegram messaging and calling app.",
        "Ritlabs.TheBat" => "Email client with advanced encryption features.",
        "Mozilla.Thunderbird" => "Mozilla's free email client.",
        "TIDAL.TIDAL" => "High-fidelity music streaming service.",
        "TikTok.TikTok" => "Short-video social media app.",
        "Transmission.Transmission" => "Lightweight BitTorrent client.",
        "Twitch.Twitch" => "Live streaming platform for gamers and creators.",
        "Meta.VCRedistAll" => "Installs all Visual C++ runtime versions (2005-2015+, x86 and x64) required by various apps.",
        "ventoy.Ventoy" => "Creates bootable USB drives holding multiple ISO images.",
        "Oracle.VirtualBox" => "Operating system virtualization software.",
        "AtomixProductions.VirtualDJ" => "DJ mixing and music software.",
        "VideoLAN.VLC" => "Universal media player supporting nearly every format.",
        "VivaldiTechnologies.Vivaldi" => "Vivaldi web browser with extensive customization options.",
        "VSCodium.VSCodium" => "Telemetry-free build of VS Code source code editor.",
        "WhatsApp.WhatsApp" => "WhatsApp messaging and calling app.",
        "RamenSoftware.Windhawk" => "Customizes Windows appearance and behavior via mods.",
        "WireGuard.WireGuard" => "Simple, fast, and modern VPN using state-of-the-art cryptography.",
        "WiseCleaner.WiseCare365" => "PC cleaning, optimization, and speed-up tool (Wise Cleaner).",
        "NoWinget.Wix" => "Wix online website builder.",
        "XnSoft.XnViewMP" => "Image viewer and converter supporting many formats.",
        "ZenBrowser.Zen" => "Web browser focused on aesthetics and productivity.",
        "Zoom.Zoom" => "Video conferencing and online meeting software.",
        "NoWinget.4KVideoDownloader" => "Downloads videos, playlists, and subtitles from YouTube and other sites in high quality.",
        "Klocman.BulkCrapUninstaller" => "Advanced tool for bulk-uninstalling programs along with their leftovers.",
        "TGRMNSoftware.BulkRenameUtility" => "Tool for renaming large batches of files at once.",
        "darktable.darktable" => "Free RAW photo editor and photography workflow application.",
        "NoWinget.EaseUSVideoDownloader" => "Downloads videos from YouTube and other popular video sites.",
        "Fastfetch-cli.Fastfetch" => "Fast command-line tool that displays system and hardware information.",
        "FreeTubeApp.FreeTube" => "Private, ad-free app for watching YouTube videos.",
        "GSmartControl.GSmartControl" => "Checks the health (SMART) status of hard drives and SSDs.",
        "ImputNet.Helium" => "Lightweight, fast Chromium-based web browser focused on privacy.",
        "NoWinget.iTubeGo" => "Downloads videos and music from YouTube and other video sites.",
        "NoWinget.JavaOpenJDK" => "Free Java runtime and development kit (OpenJDK).",
        "NoWinget.LockHunter" => "Unlocks and deletes files that are locked by other processes.",
        "nomacs.nomacs" => "Lightweight, fast image viewer with sync and editing features.",
        "CalcProgrammer1.OpenRGB" => "Open-source software for controlling RGB lighting on PC components.",
        "Pidgin.Pidgin" => "Universal chat client supporting many messaging protocols at once.",
        "PuTTY.PuTTY" => "SSH, Telnet, and serial client for connecting to remote servers.",
        "Rainmeter.Rainmeter" => "Desktop customization tool for creating and displaying configurable widgets.",
        "Genymobile.scrcpy" => "Displays and controls your Android phone's screen from your PC.",
        "NoWinget.Sefirah" => "Connects your phone to your PC (notifications, clipboard, files) – a KDE Connect-style app.",
        "ShareX.ShareX" => "Advanced screenshot, screen recording, and file-sharing tool.",
        "NoWinget.SmartRename" => "Tool for smart, bulk renaming of files.",
        "NoWinget.SnappyDriverInstaller" => "Automatically finds and installs the latest drivers for your PC hardware.",
        "KRTirtho.Spotube" => "Music player that streams from Spotify and YouTube Music without needing Premium.",
        "NoWinget.Tenacity" => "Free audio editor and recorder, a community-developed fork of Audacity.",
        "zhongyang219.TrafficMonitor" => "Real-time taskbar monitor for network speed, CPU, and memory usage.",
        "NoWinget.Wipe" => "Removes junk and temporary files, cleaning up the Windows system.",
        "NoWinget.ytDownloader" => "Simple app for downloading videos and music from YouTube and other sites.",
            _ => "No additional information available for this program.",
        }
    }
}

#[derive(Clone)]
struct CleanupTask {
    id: &'static str,
    name_pl: &'static str,
    name_en: &'static str,
    selected: bool,
}

struct WinUltimateToolkit {
    lang_is_pl: bool,
    show_eula: bool,
    show_support: bool,
    status_msg: Arc<Mutex<String>>,
    programs: Vec<(String, String, bool)>,
    cleanup_tasks: Vec<CleanupTask>,
    skip_installed: bool,
    cancel_flag: Arc<AtomicBool>,
    task_running: Arc<AtomicBool>,
    expanded_program_info: Option<String>,
}

impl Default for WinUltimateToolkit {
    fn default() -> Self {
        let mut app = Self {
            lang_is_pl: true,
            show_eula: false,
            show_support: false,
            status_msg: Arc::new(Mutex::new(String::new())),
            skip_installed: true,
            cancel_flag: Arc::new(AtomicBool::new(false)),
            task_running: Arc::new(AtomicBool::new(false)),
            expanded_program_info: None,
            cleanup_tasks: vec![
                CleanupTask { id: "standard", name_pl: "🧹 Standardowe czyszczenie (Śmieci, Cache, Telemetria, Języki)", name_en: "🧹 Standard Cleanup (Junk, Cache, Telemetry, Languages)", selected: true },
                CleanupTask { id: "optymalizacja_explorer", name_pl: "📂 Optymalizacja Eksploratora (Ten komputer, rozszerzenia, pasek)", name_en: "📂 Explorer Optimization (This PC, extensions, clean sidebar)", selected: false },
                CleanupTask { id: "start_menu", name_pl: "🪟 Optymalizacja menu Start (Rekomendacje, śledzenie)", name_en: "🪟 Optimize Start Menu (Hide recommendations and tracking)", selected: false },
                CleanupTask { id: "classic_context_menu", name_pl: "🖱 Klasyczne menu kontekstowe (Włącz/Wyłącz)", name_en: "🖱 Classic context menu (Toggle)", selected: false },
                CleanupTask { id: "sfc", name_pl: "🛠 Napraw pliki systemowe (SFC)", name_en: "🛠 Repair system files (SFC)", selected: false },
                CleanupTask { id: "dism", name_pl: "🩹 Zregeneruj obraz Windows (DISM)", name_en: "🩹 Rebuild Windows image (DISM)", selected: false },
                CleanupTask { id: "chkdsk", name_pl: "💾 Skanuj dysk w poszukiwaniu błędów (CHKDSK)", name_en: "💾 Scan disk for errors (CHKDSK)", selected: false },
                CleanupTask { id: "trim", name_pl: "⚡ Optymalizuj i przyśpiesz dysk SSD (TRIM)", name_en: "⚡ Speed up SSD performance (TRIM)", selected: false },
                CleanupTask { id: "msconfig", name_pl: "⚙ Ustawienia uruchamiania systemu (MSConfig)", name_en: "⚙ System boot settings (MSConfig)", selected: false },
                CleanupTask { id: "resmon", name_pl: "📊 Monitor zasobów (Resmon)", name_en: "📊 Resource monitor (Resmon)", selected: false },
                CleanupTask { id: "perfmon", name_pl: "📈 Historia niezawodności i awarii (Perfmon)", name_en: "📈 PC reliability and crash history (Perfmon)", selected: false },
                CleanupTask { id: "mdsched", name_pl: "🧠 Diagnostyka pamięci RAM (Mdsched)", name_en: "🧠 Windows Memory Diagnostic (Mdsched)", selected: false },
                CleanupTask { id: "dns_cloudflare", name_pl: "🌐 Zmień serwery DNS na Cloudflare", name_en: "🌐 Change DNS servers to Cloudflare", selected: false },
                CleanupTask { id: "remove_apps", name_pl: "🗑 Usuń wbudowany Bloatware", name_en: "🗑 Remove built-in Bloatware", selected: false },
                CleanupTask { id: "remove_copilot", name_pl: "🤖 Wyłącz i usuń AI Copilot", name_en: "🤖 Disable and remove AI Copilot", selected: false },
                CleanupTask { id: "remove_calc", name_pl: "🗑 Usuń Kalkulator", name_en: "🗑 Remove Calculator", selected: false },
                CleanupTask { id: "remove_edge", name_pl: "🗑 Usuń przeglądarkę Microsoft Edge", name_en: "🗑 Remove Microsoft Edge browser", selected: false },
                CleanupTask { id: "remove_notepad", name_pl: "🗑 Usuń Notatnik", name_en: "🗑 Remove Notepad", selected: false },
                CleanupTask { id: "remove_xbox_apps", name_pl: "🗑 Całkowicie wyłącz i usuń usługi Xbox", name_en: "🗑 Completely remove and disable Xbox services", selected: false },
                CleanupTask { id: "remove_widgets", name_pl: "🗑 Usuń Widżety (Wiadomości i zainteresowania)", name_en: "🗑 Remove Widgets (News and Interests)", selected: false },
                CleanupTask { id: "remove_yourphone", name_pl: "🗑 Usuń Łącze z telefonem", name_en: "🗑 Remove Phone Link", selected: false },
                CleanupTask { id: "remove_rdp", name_pl: "🗑 Wyłącz i usuń Pulpit Zdalny (RDP)", name_en: "🗑 Remove and disable Remote Desktop (RDP)", selected: false },
            ],
            programs: vec![
                ("NoWinget.ActivePresenter", "ActivePresenter", false),
                ("axpnet.AeroFTP", "AeroFTP", false),
                ("FinalWire.AIDA64.Extreme", "AIDA64", false),
                ("NoWinget.AnimatedWallpaperMaker", "Animated Wallpaper Maker", false),
                ("AnyDeskSoftwareGmbH.AnyDesk", "AnyDesk", false),
                ("AOMEI.Backupper.Standard", "AOMEI Backupper", false),
                ("AOMEI.PartitionAssistant", "AOMEI Partition Assistant", false),
                ("Apple.AppleMusic", "Apple Music", false),
                ("Apple.AppleTV", "Apple TV", false),
                ("Audacity.Audacity", "Audacity", false),
                ("NoWinget.Automize", "Automize", false),
                ("Vector35.BinaryNinja", "Binary Ninja", false),
                ("Microsoft.BingWallpaper", "Bing Wallpaper", false),
                ("Bitdefender.AntivirusFree", "Bitdefender", false),
                ("BlackmagicDesign.DaVinciResolve", "DaVinci Resolve", false),
                ("BleachBit.BleachBit", "BleachBit", false),
                ("BlenderFoundation.Blender", "Blender", false),
                ("Blix.BlueMail", "BlueMail", false),
                ("Brave.Brave", "Brave", false),
                ("TechSmith.Camtasia", "Camtasia Studio", false),
                ("Canva.Canva", "Canva", false),
                ("OpenAI.ChatGPT", "ChatGPT", false),
                ("Clonezilla.Clonezilla", "Clonezilla", false),
                ("Codex.Codex", "Codex", false),
                ("Anysphere.Cursor", "Cursor", false),
                ("DeepL.DeepL", "DeepL Translate", false),
                ("Microsoft.DirectX", "DirectX", false),
                ("Discord.Discord", "Discord", false),
                ("Disney.DisneyPlus", "Disney+", false),
                ("DMDE.DMDE", "DMDE", false),
                ("Docker.DockerDesktop", "Docker Desktop", false),
                ("DOSBox.DOSBox", "DOSBox", false),
                ("Doublecmd.Doublecmd", "Double Commander", false),
                ("NoWinget.Doxillion", "Doxillion Document Converter", false),
                ("Dropbox.Dropbox", "Dropbox", false),
                ("Duolingo.Duolingo", "Duolingo", false),
                ("EaseUS.PartitionMaster", "EaseUS Partition Master", false),
                ("EaseUS.TodoBackup", "EaseUS Todo Backup", false),
                ("NoWinget.FileConverter", "File Converter", false),
                ("TimKosse.FileZilla.Client", "FileZilla", false),
                ("FireAlpaca.FireAlpaca", "FireAlpaca", false),
                ("Mozilla.Firefox", "Firefox", false),
                ("Image-Line.FLStudio", "FL Studio", false),
                ("NoWinget.FormatFactory", "FormatFactory", false),
                ("NoWinget.freac", "fre:ac", false),
                ("FreeCAD.FreeCAD", "FreeCAD", false),
                ("FreeDownloadManager.FreeDownloadManager", "Free Download Manager", false),
                ("GIMP.GIMP.3", "GIMP", false),
                ("Git.Git", "Git", false),
                ("Google.Chrome", "Google Chrome", false),
                ("Google.GoogleDrive", "Google Drive", false),
                ("Google.PlatformTools", "Google Platform Tools", false),
                ("GParted.GParted", "GParted", false),
                ("HandBrake.HandBrake", "HandBrake", false),
                ("Hetman.Recovery", "Hetman Recovery", false),
                ("CPUID.HWMonitor", "HWMonitor", false),
                ("Inkscape.Inkscape", "Inkscape", false),
                ("Instagram.Instagram", "Instagram", false),
                ("IrfanSkiljan.IrfanView", "IrfanView", false),
                ("AppWork.JDownloader", "JDownloader", false),
                ("KDE.Kate", "Kate", false),
                ("KDE.Kdenlive", "Kdenlive", false),
                ("DominikReichl.KeePass", "KeePass", false),
                ("KiCad.KiCad", "KiCad", false),
                ("KDE.Krita", "Krita", false),
                ("TheDocumentFoundation.LibreOffice", "LibreOffice", false),
                ("LMMS.LMMS", "LMMS", false),
                ("LMStudio.LMStudio", "LM Studio", false),
                ("Mailbird.Mailbird", "Mailbird", false),
                ("MediaHuman.AudioConverter", "MediaHuman Audio Converter", false),
                ("Mega.MEGASync", "MEGAsync", false),
                ("MEGOGO.MEGOGO", "MEGOGO", false),
                ("Microsoft.PCManager", "Microsoft PC Manager", false),
                ("Microsoft.PowerToys", "Microsoft PowerToys", false),
                ("Microsoft.VisualStudio.2022.Community", "Microsoft Visual Studio", false),
                ("GNU.MidnightCommander", "Midnight Commander", false),
                ("Mixxx.Mixxx", "Mixxx", false),
                ("MartinPesch.mp3DirectCut", "mp3DirectCut", false),
                ("StevenMayall.MusicBee", "MusicBee", false),
                ("Netflix.Netflix", "Netflix", false),
                ("Nextcloud.NextcloudDesktop", "Nextcloud", false),
                ("Notepad++.Notepad++", "Notepad++", false),
                ("Nvidia.GeForceNow", "GeForce NOW", false),
                ("OBSProject.OBSStudio", "OBS Studio", false),
                ("OCBASE.OCCT", "OCCT", false),
                ("KDE.Okular", "Okular", false),
                ("Ollama.Ollama", "Ollama", false),
                ("ONLYOFFICE.DesktopEditors", "ONLYOFFICE", false),
                ("OpenClaw.OpenClawCompanion", "OpenClaw Companion", false),
                ("SST.opencode", "opencode", false),
                ("Opera.Opera", "Opera", false),
                ("Opera.OperaGX", "Opera GX", false),
                ("dotPDN.PaintDotNet", "Paint.NET", false),
                ("Perplexity.Perplexity", "Perplexity", false),
                ("Picsart.Picsart", "Picsart", false),
                ("Pinterest.Pinterest", "Pinterest", false),
                ("RedHat.Podman-Desktop", "Podman Desktop", false),
                ("Daum.PotPlayer", "PotPlayer", false),
                ("Amazon.PrimeVideo", "Prime Video", false),
                ("ProtonTechnologies.ProtonVPN", "Proton VPN", false),
                ("Qalculate.Qalculate", "Qalculate", false),
                ("qBittorrent.qBittorrent", "qBittorrent", false),
                ("Qmmp.Qmmp", "Qmmp", false),
                ("RawTherapee.RawTherapee", "RawTherapee", false),
                ("Cockos.REAPER", "REAPER", false),
                ("reMarkable.reMarkable", "reMarkable", false),
                ("Rescuezilla.Rescuezilla", "Rescuezilla", false),
                ("VSRevoGroup.RevoUninstaller", "Revo Uninstaller", false),
                ("Roblox.Roblox", "Roblox", false),
                ("Rufus.Rufus", "Rufus", false),
                ("RustDesk.RustDesk", "RustDesk", false),
                ("Meltytech.Shotcut", "Shotcut", false),
                ("PaulPacifico.ShutterEncoder", "Shutter Encoder", false),
                ("OpenWhisperSystems.Signal", "Signal", false),
                ("Smarty.Uninstaller", "Smarty Uninstaller", false),
                ("Snap.Snapchat", "Snapchat", false),
                ("Spotify.Spotify", "Spotify", false),
                ("Valve.Steam", "Steam", false),
                ("Streamlabs.StreamlabsDesktop", "Streamlabs Desktop", false),
                ("NoWinget.StrongRecovery", "StrongRecovery", false),
                ("SumatraPDF.SumatraPDF", "SumatraPDF", false),
                ("Telegram.TelegramDesktop", "Telegram", false),
                ("Ritlabs.TheBat", "The Bat!", false),
                ("Mozilla.Thunderbird", "Thunderbird", false),
                ("TIDAL.TIDAL", "TIDAL", false),
                ("TikTok.TikTok", "TikTok", false),
                ("Transmission.Transmission", "Transmission", false),
                ("Twitch.Twitch", "Twitch", false),
                ("Meta.VCRedistAll", "VCRedist (all versions)", false),
                ("ventoy.Ventoy", "Ventoy", false),
                ("Oracle.VirtualBox", "VirtualBox", false),
                ("AtomixProductions.VirtualDJ", "VirtualDJ", false),
                ("VideoLAN.VLC", "VLC", false),
                ("VivaldiTechnologies.Vivaldi", "Vivaldi", false),
                ("VSCodium.VSCodium", "VSCodium", false),
                ("WhatsApp.WhatsApp", "WhatsApp", false),
                ("RamenSoftware.Windhawk", "Windhawk", false),
                ("WireGuard.WireGuard", "WireGuard", false),
                ("WiseCleaner.WiseCare365", "Wise Cleaner", false),
                ("NoWinget.Wix", "Wix", false),
                ("XnSoft.XnViewMP", "XnViewMP", false),
                ("ZenBrowser.Zen", "Zen Browser", false),
                ("Zoom.Zoom", "Zoom Workplace", false),
                ("NoWinget.4KVideoDownloader", "4K Video Downloader", false),
                ("Klocman.BulkCrapUninstaller", "Bulk Crap Uninstaller", false),
                ("TGRMNSoftware.BulkRenameUtility", "Bulk Rename Utility", false),
                ("darktable.darktable", "darktable", false),
                ("NoWinget.EaseUSVideoDownloader", "EaseUS Video Downloader", false),
                ("Fastfetch-cli.Fastfetch", "Fastfetch", false),
                ("FreeTubeApp.FreeTube", "FreeTube", false),
                ("GSmartControl.GSmartControl", "GSmartControl", false),
                ("ImputNet.Helium", "Helium Browser", false),
                ("NoWinget.iTubeGo", "iTubeGo", false),
                ("NoWinget.JavaOpenJDK", "Java OpenJDK", false),
                ("NoWinget.LockHunter", "LockHunter", false),
                ("nomacs.nomacs", "Nomacs", false),
                ("CalcProgrammer1.OpenRGB", "OpenRGB", false),
                ("Pidgin.Pidgin", "Pidgin", false),
                ("PuTTY.PuTTY", "PuTTY", false),
                ("Rainmeter.Rainmeter", "Rainmeter", false),
                ("Genymobile.scrcpy", "scrcpy", false),
                ("NoWinget.Sefirah", "Sefirah", false),
                ("ShareX.ShareX", "ShareX", false),
                ("NoWinget.SmartRename", "SmartRename", false),
                ("NoWinget.SnappyDriverInstaller", "Snappy Driver Installer", false),
                ("KRTirtho.Spotube", "Spotube", false),
                ("NoWinget.Tenacity", "Tenacity", false),
                ("zhongyang219.TrafficMonitor", "TrafficMonitor", false),
                ("NoWinget.Wipe", "Wipe", false),
                ("NoWinget.ytDownloader", "ytDownloader", false)
            ].into_iter().map(|(w_id, name, b)| (w_id.to_string(), name.to_string(), b)).collect(),
        };

        app.load_settings();
        app
    }
}

impl WinUltimateToolkit {
    fn get_config_path() -> PathBuf {
        let mut path = std::env::temp_dir();
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            path = PathBuf::from(local_app_data);
            path.push("WinUltimateToolkit");
            let _ = fs::create_dir_all(&path);
        }
        path.push("settings.ini");
        path
    }

    fn save_settings(&self) {
        let mut config = String::new();
        config.push_str(&format!("lang_is_pl={}\n", self.lang_is_pl));
        config.push_str(&format!("skip_installed={}\n", self.skip_installed));

        for (winget_id, _, selected) in &self.programs {
            if *selected {
                config.push_str(&format!("prog={}\n", winget_id));
            }
        }

        for task in &self.cleanup_tasks {
            if task.selected {
                config.push_str(&format!("task={}\n", task.id));
            }
        }

        let _ = fs::write(Self::get_config_path(), config);
    }

    fn load_settings(&mut self) {
        if let Ok(config) = fs::read_to_string(Self::get_config_path()) {
            for (_, _, selected) in &mut self.programs { *selected = false; }
            for task in &mut self.cleanup_tasks { task.selected = false; }

            for line in config.lines() {
                if let Some((key, val)) = line.split_once('=') {
                    match key {
                        "lang_is_pl" => self.lang_is_pl = val == "true",
                        "skip_installed" => self.skip_installed = val == "true",
                        "prog" => {
                            if let Some(p) = self.programs.iter_mut().find(|(w_id, _, _)| w_id == val) {
                                p.2 = true;
                            }
                        }
                        "task" => {
                            if let Some(t) = self.cleanup_tasks.iter_mut().find(|t| t.id == val) {
                                t.selected = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn set_status(&self, msg: &str) {
        let mut status = self.status_msg.lock().unwrap();
        *status = msg.to_string();
    }

    fn get_status(&self) -> String {
        self.status_msg.lock().unwrap().clone()
    }

    fn start_tweak_thread(&mut self, ctx: &egui::Context, tweak_id: &'static str, enable: bool) {
        self.task_running.store(true, Ordering::SeqCst);
        self.cancel_flag.store(false, Ordering::SeqCst);
        let action_str = if enable {
            if self.lang_is_pl { "Włączanie..." } else { "Enabling..." }
        } else {
            if self.lang_is_pl { "Wyłączanie..." } else { "Disabling..." }
        };
        self.set_status(action_str);

        let status_clone = Arc::clone(&self.status_msg);
        let ctx_clone = ctx.clone();
        let lang_is_pl = self.lang_is_pl;
        let task_running_clone = Arc::clone(&self.task_running);

        thread::spawn(move || {
            run_tweak(tweak_id, enable);
            restart_explorer();

            *status_clone.lock().unwrap() = if lang_is_pl { String::from("Gotowe!") } else { String::from("Done!") };
            task_running_clone.store(false, Ordering::SeqCst);
            ctx_clone.request_repaint();
        });
    }

    fn start_all_tweaks_thread(&mut self, ctx: &egui::Context, enable: bool) {
        self.task_running.store(true, Ordering::SeqCst);
        self.cancel_flag.store(false, Ordering::SeqCst);
        let action_str = if enable {
            if self.lang_is_pl { "Przywracanie domyślnych usług i ustawień..." } else { "Restoring default services and settings..." }
        } else {
            if self.lang_is_pl { "Wyłączanie usług i aplikowanie optymalizacji..." } else { "Disabling services and applying tweaks..." }
        };
        self.set_status(action_str);

        let status_clone = Arc::clone(&self.status_msg);
        let ctx_clone = ctx.clone();
        let lang_is_pl = self.lang_is_pl;
        let task_running_clone = Arc::clone(&self.task_running);
        let cancel_flag = Arc::clone(&self.cancel_flag);

        thread::spawn(move || {
            let tweaks = ["visuals", "services", "wsearch", "spooler", "wia", "bluetooth", "wifi", "updates"];
            for t in tweaks {
                if cancel_flag.load(Ordering::SeqCst) { break; }
                run_tweak(t, enable);
            }

            if !cancel_flag.load(Ordering::SeqCst) {
                restart_explorer();
            }

            *status_clone.lock().unwrap() = if lang_is_pl { String::from("Gotowe!") } else { String::from("Done!") };
            task_running_clone.store(false, Ordering::SeqCst);
            ctx_clone.request_repaint();
        });
    }
}

impl eframe::App for WinUltimateToolkit {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            self.save_settings();
        }

        let is_working = self.task_running.load(Ordering::SeqCst);

        let mut style = (*ctx.style()).clone();
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);

        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(24, 24, 27);
        visuals.selection.bg_fill = egui::Color32::from_rgb(0, 120, 215);

        let rounding = egui::Rounding::from(6.0);
        visuals.widgets.inactive.rounding = rounding;
        visuals.widgets.hovered.rounding = rounding;
        visuals.widgets.active.rounding = rounding;
        visuals.widgets.noninteractive.rounding = rounding;

        ctx.set_style(style);
        ctx.set_visuals(visuals);

        egui::TopBottomPanel::top("top_panel")
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(32, 32, 36)).inner_margin(12.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("WinUltimate Toolkit").size(20.0).strong().color(egui::Color32::from_rgb(50, 205, 50)));
                ui.add_space(20.0);

                ui.label(egui::RichText::new(if self.lang_is_pl { "Język / Language:" } else { "Language / Język:" }).strong());
                if ui.selectable_label(self.lang_is_pl, "PL Polski").clicked() { self.lang_is_pl = true; }
                if ui.selectable_label(!self.lang_is_pl, "EN English").clicked() { self.lang_is_pl = false; }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(egui::RichText::new(if self.lang_is_pl { "📜 Licencja EULA" } else { "📜 EULA License" })).clicked() {
                        self.show_eula = true;
                    }
                    if ui.button(egui::RichText::new(if self.lang_is_pl { "☕ Wsparcie" } else { "☕ Support" })).clicked() {
                        self.show_support = true;
                    }
                    if ui.button(egui::RichText::new(if self.lang_is_pl { "🗃 Folder aplikacji" } else { "🗃 Apps Folder" }))
                    .on_hover_text("shell:appsFolder").clicked() {
                        open_apps_folder();
                    }
                    if ui.button(egui::RichText::new("👑 God Mode"))
                    .on_hover_text(if self.lang_is_pl { "Wszystkie ustawienia systemu w jednym folderze" } else { "All system settings in one folder" }).clicked() {
                        open_god_mode();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_status_panel")
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(32, 32, 36)).inner_margin(8.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status = self.get_status();

                if is_working {
                    ui.spinner();
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new(&status).color(egui::Color32::from_rgb(0, 200, 255)));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new(if self.lang_is_pl { "🛑 Anuluj" } else { "🛑 Cancel" }).color(egui::Color32::from_rgb(255, 100, 100))).clicked() {
                            self.cancel_flag.store(true, Ordering::SeqCst);
                            self.set_status(if self.lang_is_pl { "Anulowanie... (obecne zadanie zostanie dokończone)" } else { "Canceling... (current task will be finished)" });
                        }
                    });
                } else {
                    if status.contains("Optymalizacja zakończona") || status.contains("Optimization done") || status.contains("Deinstalacja zakończona") || status.contains("Uninstallation done") || status.contains("Zadanie zakończone") {
                        ui.label(egui::RichText::new(&status).color(egui::Color32::from_rgb(50, 205, 50)));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(egui::RichText::new(if self.lang_is_pl { "🔄 Uruchom ponownie komputer" } else { "🔄 Restart computer" }).color(egui::Color32::from_rgb(255, 100, 100))).clicked() {
                                let _ = Command::new("shutdown").args(["/r", "/t", "0"]).creation_flags(CREATE_NO_WINDOW).status();
                            }
                            ui.label(egui::RichText::new(if self.lang_is_pl { "Wymagany restart:" } else { "Restart required:" }).color(egui::Color32::from_rgb(255, 200, 100)));
                        });
                    } else if !status.is_empty() {
                        ui.label(egui::RichText::new(&status).color(egui::Color32::from_rgb(150, 150, 150)));
                    } else {
                        ui.label(egui::RichText::new(if self.lang_is_pl { "Gotowy do pracy." } else { "Ready." }).color(egui::Color32::from_rgb(100, 100, 100)));
                    }
                }
            });
        });

        if self.show_eula {
            egui::Window::new(if self.lang_is_pl { "📜 Licencja EULA" } else { "📜 EULA License" })
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .resizable(true)
            .open(&mut self.show_eula)
            .show(ctx, |ui| {
                ui.set_max_width(500.0);
                let eula_text_pl = "Oprogramowanie WinUltimate Toolkit jest dostarczane w stanie \"takim, w jakim jest\" (as-is), bez żadnych gwarancji, wyraźnych ani dorozumianych.\n\nAutor (Bartosz Szczeciński) nie ponosi odpowiedzialności za jakiekolwiek szkody, utratę danych, błędy w działaniu systemu operacyjnego ani przerwy w dostępie do usług wynikające z użytkowania tego narzędzia.\n\nOprogramowanie wykonuje zaawansowane operacje na plikach systemowych, rejestrze Windows oraz wbudowanych aplikacjach. Korzystasz z tego narzędzia na własną odpowiedzialność. Zaleca się utworzenie punktu przywracania systemu lub pełnej kopii zapasowej przed wykonaniem operacji optymalizacyjnych.\n\nInstalowane programy stron trzecich podlegają ich własnym warunkom licencyjnym.";
                let eula_text_en = "The WinUltimate Toolkit software is provided \"as-is\", without any express or implied warranty.\n\nThe author (Bartosz Szczeciński) is not responsible for any damage, data loss, operating system malfunctions, or service interruptions arising from the use of this tool.\n\nThe software performs advanced operations on system files, the Windows registry, and built-in applications. You use this tool entirely at your own risk. It is highly recommended to create a system restore point or full backup before performing any optimization operations.\n\nThird-party programs installed are subject to their own license agreements.";

                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    ui.label(egui::RichText::new(if self.lang_is_pl { eula_text_pl } else { eula_text_en }).line_height(Some(20.0)));
                });
            });
        }

        if self.show_support {
            egui::Window::new(if self.lang_is_pl { "☕ Wsparcie Twórcy" } else { "☕ Support the Creator" })
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .open(&mut self.show_support)
            .show(ctx, |ui| {
                ui.set_min_width(320.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Bartosz Szczeciński").size(18.0).strong());
                    ui.add_space(15.0);

                    ui.label(egui::RichText::new(if self.lang_is_pl {
                        "Jeśli to narzędzie okazało się przydatne i zaoszczędziło Ci trochę czasu, rozważ postawienie mi kawy, aby wesprzeć dalszy rozwój!"
                    } else {
                        "If you find this tool helpful and it saved you some time, consider buying me a coffee to support further development!"
                    }).color(egui::Color32::from_rgb(200, 200, 200)));
                    ui.add_space(15.0);

                    let btn = egui::Button::new(
                        egui::RichText::new("☕  Support the Project")
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::from_rgb(0, 0, 0)),
                    )
                    .fill(egui::Color32::from_rgb(255, 221, 0))
                    .rounding(egui::Rounding::from(8.0));

                    if ui.add_sized([260.0, 44.0], btn)
                        .on_hover_text("buymeacoffee.com/bartekszczecinski")
                        .clicked()
                    {
                        open_url("https://buymeacoffee.com/bartekszczecinski");
                    }

                    ui.add_space(15.0);
                    ui.label(egui::RichText::new(if self.lang_is_pl { "Dziękuję za wsparcie!" } else { "Thank you for your support!" }).italics().color(egui::Color32::from_rgb(150, 150, 150)));
                    ui.add_space(10.0);
                });
            });
        }

        let header_height = 175.0;

        let common_frame = egui::Frame::default()
        .fill(egui::Color32::from_rgb(24, 24, 27))
        .inner_margin(12.0);

        egui::SidePanel::left("programs_panel")
        .frame(common_frame.clone())
        .resizable(true)
        .min_width(420.0)
        .show(ctx, |ui| {
            ui.heading(if self.lang_is_pl { "📦 1. Instalator Programów" } else { "📦 1. Software Installer" });
            ui.separator();

            ui.allocate_ui(egui::vec2(ui.available_width(), header_height), |ui| {
                ui.add_enabled_ui(!is_working, |ui| {
                    ui.label(" ");

                    if ui.add_sized([ui.available_width(), 40.0], egui::Button::new(egui::RichText::new(if self.lang_is_pl { "🚀 Instaluj zaznaczone" } else { "🚀 Install selected" }).strong()).fill(egui::Color32::from_rgb(30, 130, 70))).clicked() {
                        self.task_running.store(true, Ordering::SeqCst);
                        self.cancel_flag.store(false, Ordering::SeqCst);
                        self.set_status(if self.lang_is_pl { "Rozpoczynanie instalacji..." } else { "Starting installation..." });

                        let selected_programs: Vec<(String, String)> = self.programs
                        .iter()
                        .filter(|p| p.2)
                        .map(|p| (p.0.clone(), p.1.clone()))
                        .collect();

                        let status_clone = Arc::clone(&self.status_msg);
                        let ctx_clone = ctx.clone();
                        let lang_is_pl = self.lang_is_pl;
                        let skip_installed = self.skip_installed;
                        let cancel_flag = Arc::clone(&self.cancel_flag);
                        let task_running_clone = Arc::clone(&self.task_running);

                        thread::spawn(move || {
                            for (w_id, pkg_name) in selected_programs {
                                if cancel_flag.load(Ordering::SeqCst) {
                                    *status_clone.lock().unwrap() = if lang_is_pl { String::from("🛑 Proces anulowany!") } else { String::from("🛑 Process canceled!") };
                                    ctx_clone.request_repaint();
                                    task_running_clone.store(false, Ordering::SeqCst);
                                    return;
                                }

                                if skip_installed {
                                    *status_clone.lock().unwrap() = if lang_is_pl { format!("Sprawdzanie: {}", pkg_name) } else { format!("Checking: {}", pkg_name) };
                                    ctx_clone.request_repaint();

                                    if is_package_installed(&w_id) {
                                        *status_clone.lock().unwrap() = if lang_is_pl { format!("⏭ Pominięto: {}", pkg_name) } else { format!("⏭ Skipped: {}", pkg_name) };
                                        ctx_clone.request_repaint();
                                        std::thread::sleep(std::time::Duration::from_millis(1500));
                                        continue;
                                    }
                                }

                                let success = install_package(&w_id, &pkg_name, lang_is_pl, &status_clone, &ctx_clone);

                                *status_clone.lock().unwrap() = if success {
                                    if lang_is_pl { format!("✅ Sukces: {}", pkg_name) } else { format!("✅ Success: {}", pkg_name) }
                                } else {
                                    if lang_is_pl { format!("🌐 Przekierowano do przeglądarki: {}", pkg_name) } else { format!("🌐 Redirected to browser: {}", pkg_name) }
                                };
                                ctx_clone.request_repaint();
                                std::thread::sleep(std::time::Duration::from_millis(1500));
                            }

                            if !cancel_flag.load(Ordering::SeqCst) {
                                *status_clone.lock().unwrap() = if lang_is_pl { String::from("🎉 Instalacja zakończona!") } else { String::from("🎉 Installation complete!") };
                                ctx_clone.request_repaint();
                            }
                            task_running_clone.store(false, Ordering::SeqCst);
                        });
                    }

                    ui.horizontal(|ui| {
                        let half_w = (ui.available_width() - 10.0) / 2.0;
                        if ui.add_sized([half_w, 30.0], egui::Button::new(if self.lang_is_pl { "🔄 Aktualizuj wszystkie" } else { "🔄 Update all" })).clicked() {
                            self.task_running.store(true, Ordering::SeqCst);
                            self.cancel_flag.store(false, Ordering::SeqCst);
                            self.set_status(if self.lang_is_pl { "Aktualizowanie programów..." } else { "Updating software..." });

                            let status_clone = Arc::clone(&self.status_msg);
                            let ctx_clone = ctx.clone();
                            let lang_is_pl = self.lang_is_pl;
                            let task_running_clone = Arc::clone(&self.task_running);
                            thread::spawn(move || {
                                run_updater();
                                *status_clone.lock().unwrap() = if lang_is_pl { String::from("Gotowe!") } else { String::from("Done!") };
                                task_running_clone.store(false, Ordering::SeqCst);
                                ctx_clone.request_repaint();
                            });
                        }
                        if ui.add_sized([half_w, 30.0], egui::Button::new(if self.lang_is_pl { "🌐 Zmień przeglądarkę" } else { "🌐 Change browser" })).clicked() {
                            change_default_browser();
                        }
                    });

                    ui.horizontal(|ui| {
                        let half_w = (ui.available_width() - 10.0) / 2.0;
                        if ui.add_sized([half_w, 30.0], egui::Button::new(if self.lang_is_pl { "☑ Zaznacz wszystkie" } else { "☑ Select all" })).clicked() {
                            for (_, _, is_selected) in self.programs.iter_mut() { *is_selected = true; }
                        }
                        if ui.add_sized([half_w, 30.0], egui::Button::new(if self.lang_is_pl { "☐ Odznacz wszystkie" } else { "☐ Deselect all" })).clicked() {
                            for (_, _, is_selected) in self.programs.iter_mut() { *is_selected = false; }
                        }
                    });

                    ui.checkbox(&mut self.skip_installed, egui::RichText::new(if self.lang_is_pl { "⏭ Pomiń zainstalowane programy" } else { "⏭ Skip installed programs" }).color(egui::Color32::from_rgb(255, 200, 100)));
                });
            });

            egui::Frame::dark_canvas(ui.style()).inner_margin(8.0).show(ui, |ui| {
                egui::ScrollArea::vertical().id_source("programs_scroll").auto_shrink([false, false]).show(ui, |ui| {
                    ui.add_enabled_ui(!is_working, |ui| {
                        for (winget_id, display_name, is_selected) in self.programs.iter_mut() {
                            ui.horizontal(|ui| {
                                ui.checkbox(is_selected, display_name.clone());
                                let info_label = if self.lang_is_pl { "ℹ" } else { "ℹ" };
                                let is_expanded = self.expanded_program_info.as_deref() == Some(winget_id.as_str());
                                if ui.small_button(info_label).on_hover_text(if self.lang_is_pl { "Informacje o programie" } else { "Program info" }).clicked() {
                                    if is_expanded {
                                        self.expanded_program_info = None;
                                    } else {
                                        self.expanded_program_info = Some(winget_id.clone());
                                    }
                                }
                            });
                            if self.expanded_program_info.as_deref() == Some(winget_id.as_str()) {
                                ui.indent(format!("info_{}", winget_id), |ui| {
                                    ui.label(egui::RichText::new(program_info(winget_id, self.lang_is_pl)).color(egui::Color32::from_rgb(180, 180, 180)).italics());
                                });
                            }
                        }
                    });
                });
            });
        });

        egui::SidePanel::right("tweaks_panel")
        .frame(common_frame.clone())
        .resizable(true)
        .min_width(420.0)
        .show(ctx, |ui| {
            ui.heading(if self.lang_is_pl { "⚙ 4. Optymalizator Usług" } else { "⚙ 4. Tweaks & Services" });
            ui.separator();

            ui.allocate_ui(egui::vec2(ui.available_width(), header_height), |ui| {
                ui.add_enabled_ui(!is_working, |ui| {
                    ui.label(egui::RichText::new(if self.lang_is_pl { "Zmiany aplikowane natychmiastowo:" } else { "Changes applied immediately:" }).color(egui::Color32::from_rgb(200, 200, 200)));

                    ui.horizontal(|ui| {
                        let half_w = (ui.available_width() - 10.0) / 2.0;
                        if ui.add_sized([half_w, 40.0], egui::Button::new(egui::RichText::new(if self.lang_is_pl { "🛑 Wyłącz wszystkie" } else { "🛑 Disable all" }).strong()).fill(egui::Color32::from_rgb(160, 40, 40))).clicked() {
                            self.start_all_tweaks_thread(ctx, false);
                        }
                        if ui.add_sized([half_w, 40.0], egui::Button::new(egui::RichText::new(if self.lang_is_pl { "✅ Włącz (Domyślne)" } else { "✅ Enable (Default)" }).strong()).fill(egui::Color32::from_rgb(40, 100, 180))).clicked() {
                            self.start_all_tweaks_thread(ctx, true);
                        }
                    });
                });
            });

            egui::Frame::dark_canvas(ui.style()).inner_margin(8.0).show(ui, |ui| {
                egui::ScrollArea::vertical().id_source("tweaks_scroll").auto_shrink([false, false]).show(ui, |ui| {
                    ui.add_enabled_ui(!is_working, |ui| {

                        macro_rules! tweak_row {
                            ($ui:expr, $id:expr, $pl:expr, $en:expr, $off_pl:expr, $off_en:expr, $on_pl:expr, $on_en:expr) => {
                                $ui.label(if self.lang_is_pl { $pl } else { $en });
                                $ui.horizontal(|ui| {
                                    let btn_w = (ui.available_width() - 10.0) / 2.0;
                                    if ui.add_sized([btn_w, 30.0], egui::Button::new(if self.lang_is_pl { $off_pl } else { $off_en })).clicked() {
                                        self.start_tweak_thread(ctx, $id, false);
                                    }
                                    if ui.add_sized([btn_w, 30.0], egui::Button::new(if self.lang_is_pl { $on_pl } else { $on_en })).clicked() {
                                        self.start_tweak_thread(ctx, $id, true);
                                    }
                                });
                                $ui.add_space(8.0);
                            }
                        }

                        tweak_row!(ui, "visuals", "Efekty wizualne", "System Visual Effects", "Wyłącz (Wydajność)", "Disable (Performance)", "Włącz (Domyślne)", "Enable (Default)");
                        tweak_row!(ui, "services", "Zbędne usługi (SysMain, Telemetria itp.)", "Bloat Services (SysMain, Telemetry, etc.)", "Wyłącz", "Disable", "Włącz", "Enable");
                        tweak_row!(ui, "wsearch", "Usługa wyszukiwania (Windows Search)", "Indexing Service (Windows Search)", "Wyłącz", "Disable", "Włącz", "Enable");
                        tweak_row!(ui, "spooler", "Bufor wydruku (Drukarki)", "Print Spooler (Printers)", "Wyłącz", "Disable", "Włącz", "Enable");
                        tweak_row!(ui, "wia", "Skanowanie obrazów (WIA)", "Image Acquisition (WIA)", "Wyłącz", "Disable", "Włącz", "Enable");
                        tweak_row!(ui, "bluetooth", "Moduł Bluetooth", "Bluetooth Module", "Wyłącz", "Disable", "Włącz", "Enable");
                        tweak_row!(ui, "wifi", "Autokonfiguracja sieci WLAN (Wi-Fi)", "WLAN AutoConfig Service (Wi-Fi)", "Wyłącz", "Disable", "Włącz", "Enable");
                        tweak_row!(ui, "updates", "Aktualizacje Windows", "Windows Updates", "Wstrzymaj na 180 dni", "Pause for 180 days", "Wznów / Przywróć", "Resume");

                        ui.add_space(10.0);
                        ui.separator();
                        ui.label(egui::RichText::new(if self.lang_is_pl { "⚠️ Zaawansowane (Wykluczone z 'Wyłącz wszystkie')" } else { "⚠️ Advanced (Excluded from 'Disable all')" }).color(egui::Color32::from_rgb(255, 150, 50)));
                        ui.add_space(5.0);

                        tweak_row!(ui, "hibernation", "Hibernacja systemu", "System Hibernation", "Wyłącz", "Disable", "Włącz", "Enable");
                        tweak_row!(ui, "bitlocker", "Szyfrowanie BitLocker", "BitLocker Encryption", "Wyłącz", "Disable", "Włącz", "Enable");
                        tweak_row!(ui, "uac", "Kontrola konta użytkownika (UAC)", "User Account Control (UAC)", "Wyłącz", "Disable", "Włącz", "Enable");

                    });
                });
            });
        });

        egui::CentralPanel::default()
        .frame(common_frame)
        .show(ctx, |ui| {
            ui.heading(if self.lang_is_pl { "🛠 2. Narzędzia" } else { "🛠 2. Tools" });
            ui.separator();

            egui::ScrollArea::vertical().id_source("tools_scroll").max_height(ui.available_height() * 0.45).show(ui, |ui| {
                ui.add_enabled_ui(!is_working, |ui| {
                    for task in self.cleanup_tasks.iter().filter(|t| !t.id.starts_with("remove_")) {
                        ui.horizontal(|ui| {
                            let label = if self.lang_is_pl { task.name_pl } else { task.name_en };
                            ui.label(label);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(if self.lang_is_pl { "▶ Uruchom" } else { "▶ Run" }).clicked() {
                                    self.task_running.store(true, Ordering::SeqCst);
                                    self.cancel_flag.store(false, Ordering::SeqCst);

                                    let msg = if self.lang_is_pl { format!("Uruchamianie: {}", task.name_pl) } else { format!("Running: {}", task.name_en) };
                                    self.set_status(&msg);

                                    let task_id = task.id.to_string();
                                    let status_clone = Arc::clone(&self.status_msg);
                                    let ctx_clone = ctx.clone();
                                    let lang_is_pl = self.lang_is_pl;
                                    let cancel_flag = Arc::clone(&self.cancel_flag);
                                    let task_running_clone = Arc::clone(&self.task_running);

                                    thread::spawn(move || {
                                        run_cleanup_tasks(vec![task_id], &status_clone, &ctx_clone, lang_is_pl, &cancel_flag);

                                        if !cancel_flag.load(Ordering::SeqCst) {
                                            restart_explorer();
                                            *status_clone.lock().unwrap() = if lang_is_pl { String::from("Zadanie zakończone!") } else { String::from("Task done!") };
                                            ctx_clone.request_repaint();
                                        }
                                        task_running_clone.store(false, Ordering::SeqCst);
                                    });
                                }
                            });
                        });
                        ui.add_space(4.0);
                    }
                });
            });

            ui.add_space(10.0);
            ui.heading(if self.lang_is_pl { "🗑 3. Deinstalator" } else { "🗑 3. Uninstaller" });
            ui.separator();

            ui.add_enabled_ui(!is_working, |ui| {
                if ui.add_sized([ui.available_width(), 40.0], egui::Button::new(egui::RichText::new(if self.lang_is_pl { "🚀 Usuń wybrane" } else { "🚀 Remove selected" }).strong()).fill(egui::Color32::from_rgb(160, 40, 40))).clicked() {
                    self.task_running.store(true, Ordering::SeqCst);
                    self.cancel_flag.store(false, Ordering::SeqCst);
                    self.set_status(if self.lang_is_pl { "Rozpoczynanie deinstalacji..." } else { "Starting uninstallation..." });

                    let selected_tasks: Vec<String> = self.cleanup_tasks.iter().filter(|t| t.selected && t.id.starts_with("remove_")).map(|t| t.id.to_string()).collect();
                    let status_clone = Arc::clone(&self.status_msg);
                    let ctx_clone = ctx.clone();
                    let lang_is_pl = self.lang_is_pl;
                    let cancel_flag = Arc::clone(&self.cancel_flag);
                    let task_running_clone = Arc::clone(&self.task_running);

                    thread::spawn(move || {
                        run_cleanup_tasks(selected_tasks, &status_clone, &ctx_clone, lang_is_pl, &cancel_flag);

                        if !cancel_flag.load(Ordering::SeqCst) {
                            restart_explorer();
                            *status_clone.lock().unwrap() = if lang_is_pl { String::from("Deinstalacja zakończona!") } else { String::from("Uninstallation done!") };
                            ctx_clone.request_repaint();
                        }
                        task_running_clone.store(false, Ordering::SeqCst);
                    });
                }

                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    let half_w = (ui.available_width() - 10.0) / 2.0;
                    if ui.add_sized([half_w, 30.0], egui::Button::new(if self.lang_is_pl { "☑ Zaznacz wszystkie" } else { "☑ Select all" })).clicked() {
                        for task in self.cleanup_tasks.iter_mut() {
                            if task.id.starts_with("remove_") {
                                task.selected = true;
                            }
                        }
                    }
                    if ui.add_sized([half_w, 30.0], egui::Button::new(if self.lang_is_pl { "☐ Odznacz wszystkie" } else { "☐ Deselect all" })).clicked() {
                        for task in self.cleanup_tasks.iter_mut() {
                            if task.id.starts_with("remove_") {
                                task.selected = false;
                            }
                        }
                    }
                });
                ui.add_space(5.0);
            });

            egui::Frame::dark_canvas(ui.style()).inner_margin(8.0).show(ui, |ui| {
                egui::ScrollArea::vertical().id_source("tasks_scroll").auto_shrink([false, false]).show(ui, |ui| {
                    ui.add_enabled_ui(!is_working, |ui| {
                        for task in self.cleanup_tasks.iter_mut().filter(|t| t.id.starts_with("remove_")) {
                            let label = if self.lang_is_pl { task.name_pl } else { task.name_en };
                            ui.checkbox(&mut task.selected, label);
                            ui.add_space(6.0);
                        }
                    });
                });
            });
        });
    }
}

fn get_setup_marker_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        path = PathBuf::from(local_app_data);
        path.push("WinUltimateToolkit");
        let _ = fs::create_dir_all(&path);
    }
    path.push("setup_done.flag");
    path
}

fn is_opengl_pack_installed() -> bool {
    let check = Command::new("powershell")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["-NoProfile", "-Command", "if (Get-AppxPackage *OpenCL*) { Write-Output 'INSTALLED' }"])
    .output();

    match check {
        Ok(out) => String::from_utf8_lossy(&out.stdout).contains("INSTALLED"),
        Err(_) => false,
    }
}

fn run_first_time_setup() {
    let need_winget = !is_winget_available();
    let need_opengl = !is_opengl_pack_installed();
    let need_scoop = !is_scoop_available();
    let need_choco = !is_choco_available();

    if !need_winget && !need_opengl && !need_scoop && !need_choco {
        return;
    }

    let ps_template = r#"
    Add-Type -AssemblyName System.Windows.Forms
    Add-Type -AssemblyName System.Drawing

    $needWinget = $__NEED_WINGET__
    $needOpenGL = $__NEED_OPENGL__
    $needScoop = $__NEED_SCOOP__
    $needChoco = $__NEED_CHOCO__

    $sync = [Hashtable]::Synchronized(@{ Status = "Przygotowywanie instalacji...`nPreparing installation..."; Done = $false })

    $form = New-Object System.Windows.Forms.Form
    $form.Text = "WinUltimate Toolkit - Instalacja / Installation"
    $form.Size = New-Object System.Drawing.Size(480,180)
    $form.StartPosition = "CenterScreen"
    $form.FormBorderStyle = "FixedDialog"
    $form.ControlBox = $false
    $form.MaximizeBox = $false
    $form.MinimizeBox = $false
    $form.TopMost = $true

    $lbl = New-Object System.Windows.Forms.Label
    $lbl.Text = $sync.Status
    $lbl.AutoSize = $false
    $lbl.Size = New-Object System.Drawing.Size(440,55)
    $lbl.Location = New-Object System.Drawing.Point(15,15)
    $form.Controls.Add($lbl)

    $pb = New-Object System.Windows.Forms.ProgressBar
    $pb.Style = "Marquee"
    $pb.Size = New-Object System.Drawing.Size(440,20)
    $pb.Location = New-Object System.Drawing.Point(15,78)
    $form.Controls.Add($pb)

    $lbl2 = New-Object System.Windows.Forms.Label
    $lbl2.Text = "To okno zamknie się automatycznie po zakończeniu.`nThis window will close automatically when finished."
    $lbl2.AutoSize = $false
    $lbl2.Size = New-Object System.Drawing.Size(440,40)
    $lbl2.Location = New-Object System.Drawing.Point(15,108)
    $lbl2.ForeColor = [System.Drawing.Color]::Gray
    $form.Controls.Add($lbl2)

    $rs = [runspacefactory]::CreateRunspace()
    $rs.ApartmentState = "STA"
    $rs.ThreadOptions = "ReuseThread"
    $rs.Open()
    $rs.SessionStateProxy.SetVariable("sync", $sync)
    $rs.SessionStateProxy.SetVariable("needWinget", $needWinget)
    $rs.SessionStateProxy.SetVariable("needOpenGL", $needOpenGL)
    $rs.SessionStateProxy.SetVariable("needScoop", $needScoop)
    $rs.SessionStateProxy.SetVariable("needChoco", $needChoco)

    $psInst = [PowerShell]::Create()
    $psInst.Runspace = $rs

    [void]$psInst.AddScript({
        try {
            if ($needWinget) {
                $sync.Status = "Trwa pobieranie i instalacja menedżera pakietów Winget...`nDownloading and installing the Winget package manager..."
                $progressPreference = 'silentlyContinue'
                [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
                $temp = $env:TEMP
                Invoke-WebRequest -Uri "https://aka.ms/windowsappsdk/1.8/latest/windowsappruntimeinstall-x64.exe" -OutFile "$temp\winapprt.exe" -UseBasicParsing
                Start-Process -FilePath "$temp\winapprt.exe" -ArgumentList "--quiet" -Wait -NoNewWindow
                Invoke-WebRequest -Uri "https://github.com/microsoft/microsoft-ui-xaml/releases/download/v2.8.6/Microsoft.UI.Xaml.2.8.x64.appx" -OutFile "$temp\uixaml.appx" -ErrorAction SilentlyContinue
                Add-AppxPackage -Path "$temp\uixaml.appx" -ErrorAction SilentlyContinue
                Invoke-WebRequest -Uri "https://aka.ms/Microsoft.VCLibs.x64.14.00.Desktop.appx" -OutFile "$temp\vclibs_desktop.appx" -UseBasicParsing
                Add-AppxPackage -Path "$temp\vclibs_desktop.appx" -ErrorAction SilentlyContinue
                $wingetUrl = "https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"
                Invoke-WebRequest -Uri $wingetUrl -OutFile "$temp\winget.msixbundle" -UseBasicParsing
                Add-AppxPackage -Path "$temp\winget.msixbundle"
            }

            if ($needOpenGL) {
                $sync.Status = "Trwa przygotowywanie pakietów...`nPreparing packages..."
                Start-Process winget -ArgumentList "install --id 9NQPSL29BFFF --exact --source msstore --silent --accept-package-agreements --accept-source-agreements" -WindowStyle Hidden -Wait
            }

            if ($needScoop) {
                $sync.Status = "Trwa instalacja menedżera pakietów Scoop...`nInstalling the Scoop package manager..."
                try {
                    Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force -ErrorAction SilentlyContinue
                    Invoke-Expression "& {$(Invoke-RestMethod -Uri 'https://get.scoop.sh')} -RunAsAdmin"
                    $scoopCmd = "$env:USERPROFILE\scoop\shims\scoop.cmd"
                    if (Test-Path $scoopCmd) {
                        Start-Process -FilePath $scoopCmd -ArgumentList "bucket add extras" -WindowStyle Hidden -Wait -ErrorAction SilentlyContinue
                    }
                } catch {}
            }

            if ($needChoco) {
                $sync.Status = "Trwa instalacja menedżera pakietów Chocolatey...`nInstalling the Chocolatey package manager..."
                try {
                    Set-ExecutionPolicy Bypass -Scope Process -Force -ErrorAction SilentlyContinue
                    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
                    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
                } catch {}
            }
        } finally {
            $sync.Done = $true
        }
    })

    $handle = $psInst.BeginInvoke()

    $timer = New-Object System.Windows.Forms.Timer
    $timer.Interval = 300
    $timer.Add_Tick({
        $lbl.Text = $sync.Status
        if ($sync.Done) {
            $timer.Stop()
            $form.Close()
        }
    })
    $form.Add_Shown({ $timer.Start() })
    $form.ShowDialog() | Out-Null

    $psInst.EndInvoke($handle) | Out-Null
    $psInst.Dispose()
    $rs.Close()
    "#;

    let ps_script = ps_template
        .replace("__NEED_WINGET__", if need_winget { "true" } else { "false" })
        .replace("__NEED_OPENGL__", if need_opengl { "true" } else { "false" })
        .replace("__NEED_SCOOP__", if need_scoop { "true" } else { "false" })
        .replace("__NEED_CHOCO__", if need_choco { "true" } else { "false" });

    let _ = Command::new("powershell")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &ps_script])
    .status();
}

fn main() -> eframe::Result<()> {
    let marker_path = get_setup_marker_path();

    if !marker_path.exists() {
        run_first_time_setup();

        let _ = fs::write(&marker_path, "1");
    }

    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([1400.0, 750.0])
        .with_min_inner_size([1200.0, 600.0])
        .with_icon(std::sync::Arc::new(load_icon())),
        default_theme: eframe::Theme::Dark,
            follow_system_theme: false,
            ..Default::default()
    };

    options.wgpu_options.supported_backends = eframe::wgpu::Backends::GL;

    eframe::run_native(
        "WinUltimate Toolkit",
        options,
        Box::new(|_cc| Ok(Box::new(WinUltimateToolkit::default()) as Box<dyn eframe::App>)),
    )
}

fn is_winget_available() -> bool {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let path = std::path::PathBuf::from(local_app_data).join("Microsoft\\WindowsApps\\winget.exe");
        if path.exists() {
            return true;
        }
    }

    Command::new("winget")
    .arg("--version")
    .creation_flags(CREATE_NO_WINDOW)
    .status()
    .map_or(false, |s| s.success())
}

fn scoop_shims_path() -> Option<PathBuf> {
    if let Ok(scoop_env) = std::env::var("SCOOP") {
        let path = PathBuf::from(scoop_env).join("shims").join("scoop.cmd");
        if path.exists() {
            return Some(path);
        }
    }
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        let path = PathBuf::from(user_profile).join("scoop").join("shims").join("scoop.cmd");
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn is_scoop_available() -> bool {
    if scoop_shims_path().is_some() {
        return true;
    }

    Command::new("scoop")
    .arg("--version")
    .creation_flags(CREATE_NO_WINDOW)
    .status()
    .map_or(false, |s| s.success())
}

fn scoop_command() -> Command {
    match scoop_shims_path() {
        Some(path) => Command::new(path),
        None => Command::new("scoop"),
    }
}

fn choco_exe_path() -> PathBuf {
    let program_data = std::env::var("ProgramData").unwrap_or_else(|_| "C:\\ProgramData".to_string());
    PathBuf::from(program_data).join("chocolatey").join("bin").join("choco.exe")
}

fn is_choco_available() -> bool {
    if choco_exe_path().exists() {
        return true;
    }

    Command::new("choco")
    .arg("--version")
    .creation_flags(CREATE_NO_WINDOW)
    .status()
    .map_or(false, |s| s.success())
}

fn choco_command() -> Command {
    let path = choco_exe_path();
    if path.exists() {
        Command::new(path)
    } else {
        Command::new("choco")
    }
}

fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut last_was_sep = true;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_was_sep = false;
        } else if !last_was_sep {
            out.push('-');
            last_was_sep = true;
        }
    }

    out.trim_matches('-').to_string()
}

fn candidate_package_names(winget_id: &str, pkg_name: &str) -> Vec<String> {
    let mut candidates = Vec::new();

    if let Some(part) = winget_id.rsplit('.').next() {
        let slug = slugify(part);
        if !slug.is_empty() {
            candidates.push(slug);
        }
    }

    let name_slug = slugify(pkg_name);
    if !name_slug.is_empty() && !candidates.contains(&name_slug) {
        candidates.push(name_slug);
    }

    candidates
}

fn try_scoop_install(candidates: &[String], pkg_name: &str, lang_is_pl: bool, status: &Arc<Mutex<String>>, ctx: &egui::Context) -> bool {
    let update_status = |msg: &str| {
        *status.lock().unwrap() = msg.to_string();
        ctx.request_repaint();
    };

    for candidate in candidates {
        update_status(&if lang_is_pl {
            format!("Instalowanie (Scoop): {}", pkg_name)
        } else {
            format!("Installing (Scoop): {}", pkg_name)
        });

        let success = scoop_command()
        .creation_flags(CREATE_NO_WINDOW)
        .args(["install", candidate.as_str()])
        .status()
        .map_or(false, |s| s.success());

        if success {
            return true;
        }
    }

    false
}

fn try_choco_install(candidates: &[String], pkg_name: &str, lang_is_pl: bool, status: &Arc<Mutex<String>>, ctx: &egui::Context) -> bool {
    let update_status = |msg: &str| {
        *status.lock().unwrap() = msg.to_string();
        ctx.request_repaint();
    };

    for candidate in candidates {
        update_status(&if lang_is_pl {
            format!("Instalowanie (Chocolatey): {}", pkg_name)
        } else {
            format!("Installing (Chocolatey): {}", pkg_name)
        });

        let success = choco_command()
        .creation_flags(CREATE_NO_WINDOW)
        .args(["install", candidate.as_str(), "-y", "--no-progress", "--accept-license"])
        .status()
        .map_or(false, |s| s.success());

        if success {
            return true;
        }
    }

    false
}

fn known_scoop_id(winget_id: &str) -> Option<&'static str> {
    match winget_id {
        "AnyDeskSoftwareGmbH.AnyDesk" => Some("anydesk"),
        "Audacity.Audacity" => Some("audacity"),
        "BleachBit.BleachBit" => Some("bleachbit"),
        "BlenderFoundation.Blender" => Some("blender"),
        "Brave.Brave" => Some("brave"),
        "Anysphere.Cursor" => Some("cursor"),
        "DeepL.DeepL" => Some("deepl"),
        "Discord.Discord" => Some("discord"),
        "Docker.DockerDesktop" => Some("docker-desktop"),
        "DOSBox.DOSBox" => Some("dosbox"),
        "Doublecmd.Doublecmd" => Some("doublecmd"),
        "Dropbox.Dropbox" => Some("dropbox"),
        "TimKosse.FileZilla.Client" => Some("filezilla"),
        "Mozilla.Firefox" => Some("firefox"),
        "FreeCAD.FreeCAD" => Some("freecad"),
        "GIMP.GIMP.3" => Some("gimp"),
        "Git.Git" => Some("git"),
        "Google.Chrome" => Some("googlechrome"),
        "Google.PlatformTools" => Some("adb"),
        "HandBrake.HandBrake" => Some("handbrake"),
        "CPUID.HWMonitor" => Some("hwmonitor"),
        "Inkscape.Inkscape" => Some("inkscape"),
        "IrfanSkiljan.IrfanView" => Some("irfanview"),
        "KDE.Kdenlive" => Some("kdenlive"),
        "DominikReichl.KeePass" => Some("keepass"),
        "KiCad.KiCad" => Some("kicad"),
        "KDE.Krita" => Some("krita"),
        "TheDocumentFoundation.LibreOffice" => Some("libreoffice"),
        "LMMS.LMMS" => Some("lmms"),
        "LMStudio.LMStudio" => Some("lm-studio"),
        "Mega.MEGASync" => Some("megasync"),
        "Microsoft.PowerToys" => Some("powertoys"),
        "Mixxx.Mixxx" => Some("mixxx"),
        "StevenMayall.MusicBee" => Some("musicbee"),
        "Nextcloud.NextcloudDesktop" => Some("nextcloud-client"),
        "Notepad++.Notepad++" => Some("notepadplusplus"),
        "OBSProject.OBSStudio" => Some("obs-studio"),
        "Ollama.Ollama" => Some("ollama"),
        "ONLYOFFICE.DesktopEditors" => Some("onlyoffice"),
        "Opera.Opera" => Some("opera"),
        "Opera.OperaGX" => Some("opera-gx"),
        "dotPDN.PaintDotNet" => Some("paint.net"),
        "RedHat.Podman-Desktop" => Some("podman-desktop"),
        "Daum.PotPlayer" => Some("potplayer"),
        "ProtonTechnologies.ProtonVPN" => Some("protonvpn"),
        "qBittorrent.qBittorrent" => Some("qbittorrent"),
        "RawTherapee.RawTherapee" => Some("rawtherapee"),
        "Cockos.REAPER" => Some("reaper"),
        "Rufus.Rufus" => Some("rufus"),
        "RustDesk.RustDesk" => Some("rustdesk"),
        "Meltytech.Shotcut" => Some("shotcut"),
        "OpenWhisperSystems.Signal" => Some("signal"),
        "Spotify.Spotify" => Some("spotify"),
        "Valve.Steam" => Some("steam"),
        "SumatraPDF.SumatraPDF" => Some("sumatrapdf"),
        "Telegram.TelegramDesktop" => Some("telegram"),
        "Mozilla.Thunderbird" => Some("thunderbird"),
        "Transmission.Transmission" => Some("transmission"),
        "ventoy.Ventoy" => Some("ventoy"),
        "Oracle.VirtualBox" => Some("virtualbox"),
        "VideoLAN.VLC" => Some("vlc"),
        "VivaldiTechnologies.Vivaldi" => Some("vivaldi"),
        "VSCodium.VSCodium" => Some("vscodium"),
        "RamenSoftware.Windhawk" => Some("windhawk"),
        "WireGuard.WireGuard" => Some("wireguard"),
        "XnSoft.XnViewMP" => Some("xnviewmp"),
        "ZenBrowser.Zen" => Some("zen-browser"),
        "Zoom.Zoom" => Some("zoom"),
        "Klocman.BulkCrapUninstaller" => Some("bulk-crap-uninstaller"),
        "darktable.darktable" => Some("darktable"),
        "Fastfetch-cli.Fastfetch" => Some("fastfetch"),
        "FreeTubeApp.FreeTube" => Some("freetube"),
        "GSmartControl.GSmartControl" => Some("gsmartcontrol"),
        "nomacs.nomacs" => Some("nomacs"),
        "CalcProgrammer1.OpenRGB" => Some("openrgb"),
        "Pidgin.Pidgin" => Some("pidgin"),
        "PuTTY.PuTTY" => Some("putty"),
        "Rainmeter.Rainmeter" => Some("rainmeter"),
        "Genymobile.scrcpy" => Some("scrcpy"),
        "ShareX.ShareX" => Some("sharex"),
        "KRTirtho.Spotube" => Some("spotube"),
        _ => None,
    }
}

fn known_choco_id(winget_id: &str) -> Option<&'static str> {
    match winget_id {
        "NoWinget.ActivePresenter" => Some("activepresenter"),
        "FinalWire.AIDA64.Extreme" => Some("aida64-extreme"),
        "NoWinget.FileConverter" => Some("file-converter"),
        "NoWinget.freac" => Some("freac"),
        "NoWinget.SnappyDriverInstaller" => Some("sdio"),
        "NoWinget.Tenacity" => Some("tenacity"),
        "NoWinget.Wix" => Some("wixtoolset"),
        "NoWinget.SmartRename" => Some("smartrename"),
        "NoWinget.ytDownloader" => Some("ytdownloader"),
        "AnyDeskSoftwareGmbH.AnyDesk" => Some("anydesk"),
        "Audacity.Audacity" => Some("audacity"),
        "BleachBit.BleachBit" => Some("bleachbit"),
        "BlenderFoundation.Blender" => Some("blender"),
        "Brave.Brave" => Some("brave"),
        "TechSmith.Camtasia" => Some("camtasia"),
        "Clonezilla.Clonezilla" => Some("clonezilla"),
        "Anysphere.Cursor" => Some("cursor"),
        "DeepL.DeepL" => Some("deepl"),
        "Microsoft.DirectX" => Some("directx"),
        "Discord.Discord" => Some("discord"),
        "Docker.DockerDesktop" => Some("docker-desktop"),
        "DOSBox.DOSBox" => Some("dosbox"),
        "Doublecmd.Doublecmd" => Some("doublecmd"),
        "Dropbox.Dropbox" => Some("dropbox"),
        "TimKosse.FileZilla.Client" => Some("filezilla"),
        "Mozilla.Firefox" => Some("firefox"),
        "NoWinget.FormatFactory" => Some("formatfactory"),
        "FreeCAD.FreeCAD" => Some("freecad"),
        "FreeDownloadManager.FreeDownloadManager" => Some("freedownloadmanager"),
        "GIMP.GIMP.3" => Some("gimp"),
        "Git.Git" => Some("git"),
        "Google.Chrome" => Some("googlechrome"),
        "Google.PlatformTools" => Some("adb"),
        "HandBrake.HandBrake" => Some("handbrake"),
        "CPUID.HWMonitor" => Some("hwmonitor"),
        "Inkscape.Inkscape" => Some("inkscape"),
        "IrfanSkiljan.IrfanView" => Some("irfanview"),
        "AppWork.JDownloader" => Some("jdownloader"),
        "KDE.Kdenlive" => Some("kdenlive"),
        "DominikReichl.KeePass" => Some("keepass"),
        "KiCad.KiCad" => Some("kicad"),
        "KDE.Krita" => Some("krita"),
        "TheDocumentFoundation.LibreOffice" => Some("libreoffice-fresh"),
        "LMMS.LMMS" => Some("lmms"),
        "Mega.MEGASync" => Some("megasync"),
        "Microsoft.PowerToys" => Some("powertoys"),
        "Microsoft.VisualStudio.2022.Community" => Some("visualstudio2022community"),
        "Mixxx.Mixxx" => Some("mixxx"),
        "StevenMayall.MusicBee" => Some("musicbee"),
        "Nextcloud.NextcloudDesktop" => Some("nextcloud-client"),
        "Notepad++.Notepad++" => Some("notepadplusplus.install"),
        "OBSProject.OBSStudio" => Some("obs-studio"),
        "Ollama.Ollama" => Some("ollama"),
        "ONLYOFFICE.DesktopEditors" => Some("onlyoffice"),
        "Opera.Opera" => Some("opera"),
        "Opera.OperaGX" => Some("opera-gx"),
        "dotPDN.PaintDotNet" => Some("paint.net"),
        "RedHat.Podman-Desktop" => Some("podman-desktop"),
        "Daum.PotPlayer" => Some("potplayer"),
        "ProtonTechnologies.ProtonVPN" => Some("protonvpn"),
        "qBittorrent.qBittorrent" => Some("qbittorrent"),
        "RawTherapee.RawTherapee" => Some("rawtherapee"),
        "Cockos.REAPER" => Some("reaper"),
        "VSRevoGroup.RevoUninstaller" => Some("revo-uninstaller"),
        "Rufus.Rufus" => Some("rufus"),
        "RustDesk.RustDesk" => Some("rustdesk.install"),
        "Meltytech.Shotcut" => Some("shotcut"),
        "OpenWhisperSystems.Signal" => Some("signal"),
        "Spotify.Spotify" => Some("spotify"),
        "Valve.Steam" => Some("steam"),
        "SumatraPDF.SumatraPDF" => Some("sumatrapdf"),
        "Telegram.TelegramDesktop" => Some("telegram"),
        "Mozilla.Thunderbird" => Some("thunderbird"),
        "Transmission.Transmission" => Some("transmission"),
        "ventoy.Ventoy" => Some("ventoy"),
        "Oracle.VirtualBox" => Some("virtualbox"),
        "VideoLAN.VLC" => Some("vlc"),
        "VivaldiTechnologies.Vivaldi" => Some("vivaldi"),
        "VSCodium.VSCodium" => Some("vscodium"),
        "WireGuard.WireGuard" => Some("wireguard"),
        "XnSoft.XnViewMP" => Some("xnviewmp"),
        "Zoom.Zoom" => Some("zoom"),
        "NoWinget.4KVideoDownloader" => Some("4k-video-downloader"),
        "Klocman.BulkCrapUninstaller" => Some("bulk-crap-uninstaller"),
        "TGRMNSoftware.BulkRenameUtility" => Some("bulkrenameutility"),
        "darktable.darktable" => Some("darktable"),
        "Fastfetch-cli.Fastfetch" => Some("fastfetch"),
        "FreeTubeApp.FreeTube" => Some("freetube"),
        "GSmartControl.GSmartControl" => Some("gsmartcontrol"),
        "NoWinget.JavaOpenJDK" => Some("temurin"),
        "NoWinget.LockHunter" => Some("lockhunter"),
        "nomacs.nomacs" => Some("nomacs"),
        "CalcProgrammer1.OpenRGB" => Some("openrgb"),
        "Pidgin.Pidgin" => Some("pidgin"),
        "PuTTY.PuTTY" => Some("putty"),
        "Rainmeter.Rainmeter" => Some("rainmeter"),
        "Genymobile.scrcpy" => Some("scrcpy"),
        "ShareX.ShareX" => Some("sharex"),
        _ => None,
    }
}

fn scoop_candidates(winget_id: &str, pkg_name: &str) -> Vec<String> {
    let mut candidates = Vec::new();

    if let Some(known) = known_scoop_id(winget_id) {
        candidates.push(known.to_string());
    }

    for guess in candidate_package_names(winget_id, pkg_name) {
        if !candidates.contains(&guess) {
            candidates.push(guess);
        }
    }

    candidates
}

fn choco_candidates(winget_id: &str, pkg_name: &str) -> Vec<String> {
    let mut candidates = Vec::new();

    if let Some(known) = known_choco_id(winget_id) {
        candidates.push(known.to_string());
    }

    for guess in candidate_package_names(winget_id, pkg_name) {
        if !candidates.contains(&guess) {
            candidates.push(guess);
        }
    }

    candidates
}

fn is_package_installed(pkg_id: &str) -> bool {
    if pkg_id == VCREDIST_META_ID {
        return VCREDIST_IDS.iter().all(|id| is_package_installed(id));
    }

    let output = Command::new("winget")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["list", "--id", pkg_id, "--exact", "--accept-source-agreements"])
    .output();

    if let Ok(out) = output {
        out.status.success()
    } else {
        false
    }
}

fn known_direct_exe_url(winget_id: &str) -> Option<&'static str> {
    match winget_id {
        "NoWinget.AnimatedWallpaperMaker" => Some("http://download.desktoppaints.com/wp_maker.exe"),
        "NoWinget.Automize" => Some("https://www.hiteksoftware.com/mize/install/14x/windows/mize.exe"),
        "NoWinget.Doxillion" => Some("https://www.nchsoftware.com/documentconvert/doxillionsetup.exe"),
        "NoWinget.EaseUSVideoDownloader" => Some("https://down.easeus.com/product/video_downloader"),
        "NoWinget.iTubeGo" => Some("https://itubego.com/download/?product=itubego.exe"),
        "NoWinget.StrongRecovery" => Some("https://www.strongrecovery.com/xdownload.php?xfile=StrongRecovery.exe"),
        _ => None,
    }
}

fn known_msstore_id(winget_id: &str) -> Option<&'static str> {
    match winget_id {
        "NoWinget.Sefirah" => Some("9PJV6D1JPG0H"),
        _ => None,
    }
}

fn try_msstore_install(store_id: &str, pkg_name: &str, lang_is_pl: bool, status: &Arc<Mutex<String>>, ctx: &egui::Context) -> bool {
    let update_status = |msg: &str| {
        *status.lock().unwrap() = msg.to_string();
        ctx.request_repaint();
    };

    update_status(&if lang_is_pl {
        format!("Instalowanie (Microsoft Store): {}", pkg_name)
    } else {
        format!("Installing (Microsoft Store): {}", pkg_name)
    });

    Command::new("winget")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["install", "--id", store_id, "-e", "--source", "msstore", "--accept-package-agreements", "--accept-source-agreements"])
    .status()
    .map_or(false, |s| s.success())
}

fn download_file(url: &str, dest: &PathBuf) -> bool {
    let ps_cmd = format!(
        "$ProgressPreference = 'SilentlyContinue'; try {{ Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing }} catch {{ exit 1 }}",
        url,
        dest.display()
    );

    let success = Command::new("powershell")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["-NoProfile", "-Command", &ps_cmd])
    .status()
    .map_or(false, |s| s.success());

    success && dest.exists()
}

fn run_downloaded_installer(path: &PathBuf) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();

    if ext == "msi" {
        Command::new("msiexec")
        .args(["/i", &path.display().to_string(), "/qn", "/norestart"])
        .status()
        .map_or(false, |s| s.success())
    } else {
        Command::new(path)
        .status()
        .map_or(false, |s| s.success())
    }
}

fn try_direct_download_install(winget_id: &str, pkg_name: &str, lang_is_pl: bool, status: &Arc<Mutex<String>>, ctx: &egui::Context) -> bool {
    let update_status = |msg: &str| {
        *status.lock().unwrap() = msg.to_string();
        ctx.request_repaint();
    };

    let url = match known_direct_exe_url(winget_id) {
        Some(u) => u,
        None => return false,
    };

    update_status(&if lang_is_pl {
        format!("Pobieranie instalatora: {}", pkg_name)
    } else {
        format!("Downloading installer: {}", pkg_name)
    });

    let file_name = url.rsplit('/').next().unwrap_or("installer.exe");
    let dest = std::env::temp_dir().join(file_name);

    if !download_file(url, &dest) {
        return false;
    }

    update_status(&if lang_is_pl {
        format!("Instalowanie: {}", pkg_name)
    } else {
        format!("Installing: {}", pkg_name)
    });

    let success = run_downloaded_installer(&dest);
    let _ = fs::remove_file(&dest);
    success
}

fn install_package(winget_id: &str, pkg_name: &str, lang_is_pl: bool, status: &Arc<Mutex<String>>, ctx: &egui::Context) -> bool {
    let update_status = |msg: &str| {
        *status.lock().unwrap() = msg.to_string();
        ctx.request_repaint();
    };

    if winget_id == VCREDIST_META_ID {
        let mut all_ok = true;
        for (i, sub_id) in VCREDIST_IDS.iter().enumerate() {
            update_status(&if lang_is_pl {
                format!("Instalowanie (Winget): {} ({}/{})", pkg_name, i + 1, VCREDIST_IDS.len())
            } else {
                format!("Installing (Winget): {} ({}/{})", pkg_name, i + 1, VCREDIST_IDS.len())
            });

            let success = Command::new("winget")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["install", "--id", sub_id, "-e", "--source", "winget", "--silent", "--accept-package-agreements", "--accept-source-agreements"])
            .status()
            .map_or(false, |s| s.success());

            if !success {
                all_ok = false;
            }
        }

        if !all_ok {
            update_status(&if lang_is_pl {
                format!("Błąd pobierania. Otwieranie strony: {}", pkg_name)
            } else {
                format!("Download failed. Opening website: {}", pkg_name)
            });
            open_url("https://learn.microsoft.com/en-US/cpp/windows/latest-supported-vc-redist");
        }

        return all_ok;
    }

    update_status(&if lang_is_pl { format!("Instalowanie (Winget): {}", pkg_name) } else { format!("Installing (Winget): {}", pkg_name) });
    let success_winget = Command::new("winget")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["install", "--id", winget_id, "-e", "--source", "winget", "--silent", "--accept-package-agreements", "--accept-source-agreements"])
    .status()
    .map_or(false, |s| s.success());

    if success_winget {
        return true;
    }

    if let Some(store_id) = known_msstore_id(winget_id) {
        if try_msstore_install(store_id, pkg_name, lang_is_pl, status, ctx) {
            return true;
        }
    }

    let scoop_cands = scoop_candidates(winget_id, pkg_name);
    let choco_cands = choco_candidates(winget_id, pkg_name);

    if is_scoop_available() && try_scoop_install(&scoop_cands, pkg_name, lang_is_pl, status, ctx) {
        return true;
    }

    if is_choco_available() && try_choco_install(&choco_cands, pkg_name, lang_is_pl, status, ctx) {
        return true;
    }

    if try_direct_download_install(winget_id, pkg_name, lang_is_pl, status, ctx) {
        return true;
    }

    update_status(&if lang_is_pl {
        format!("Błąd pobierania. Otwieranie strony: {}", pkg_name)
    } else {
        format!("Download failed. Opening website: {}", pkg_name)
    });

    let url = match winget_id {
        "AnyDeskSoftwareGmbH.AnyDesk" => "https://anydesk.com/en/downloads/windows".to_string(),
        "Apple.AppleMusic" => "https://apps.microsoft.com/detail/9pfhdd62mxs1".to_string(),
        "Apple.AppleTV" => "https://apps.microsoft.com/detail/9pf4kz2vn4wa".to_string(),
        "Audacity.Audacity" => "https://www.audacityteam.org/download/".to_string(),
        "Vector35.BinaryNinja" => "https://binary.ninja/free/".to_string(),
        "Bitdefender.AntivirusFree" => "https://www.bitdefender.com/solutions/free.html".to_string(),
        "BlackmagicDesign.DaVinciResolve" => "https://www.blackmagicdesign.com/products/davinciresolve".to_string(),
        "BleachBit.BleachBit" => "https://www.bleachbit.org/download/windows".to_string(),
        "BlenderFoundation.Blender" => "https://www.blender.org/download/".to_string(),
        "Blix.BlueMail" => "https://bluemail.me/windows".to_string(),
        "Brave.Brave" => "https://brave.com/download/".to_string(),
        "Canva.Canva" => "https://www.canva.com/download/windows/".to_string(),
        "OpenAI.ChatGPT" => "https://chat.openai.com/".to_string(),
        "Clonezilla.Clonezilla" => "https://clonezilla.org/downloads.php".to_string(),
        "Codex.Codex" => "https://github.com/".to_string(),
        "Anysphere.Cursor" => "https://cursor.com/".to_string(),
        "DeepL.DeepL" => "https://www.deepl.com/en/app/".to_string(),
        "Microsoft.DirectX" => "https://www.microsoft.com/en-us/download/details.aspx?id=35".to_string(),
        "Discord.Discord" => "https://discord.com/download".to_string(),
        "Disney.DisneyPlus" => "https://apps.microsoft.com/detail/9nxqxxlfst89".to_string(),
        "DMDE.DMDE" => "https://dmde.com/download.html".to_string(),
        "Docker.DockerDesktop" => "https://www.docker.com/products/docker-desktop/".to_string(),
        "Doublecmd.Doublecmd" => "https://doublecmd.sourceforge.io/".to_string(),
        "Duolingo.Duolingo" => "https://apps.microsoft.com/detail/9wzdncrcv5xn".to_string(),
        "FireAlpaca.FireAlpaca" => "https://firealpaca.com/download/".to_string(),
        "Mozilla.Firefox" => "https://www.mozilla.org/en-US/firefox/new/".to_string(),
        "Image-Line.FLStudio" => "https://www.image-line.com/fl-studio-download/".to_string(),
        "FreeCAD.FreeCAD" => "https://www.freecad.org/downloads.php".to_string(),
        "FreeDownloadManager.FreeDownloadManager" => "https://www.freedownloadmanager.org/download.htm".to_string(),
        "GIMP.GIMP.3" => "https://www.gimp.org/downloads/".to_string(),
        "Google.Chrome" => "https://www.google.com/chrome/".to_string(),
        "Google.PlatformTools" => "https://developer.android.com/tools/releases/platform-tools".to_string(),
        "GParted.GParted" => "https://gparted.org/download.php".to_string(),
        "Hetman.Recovery" => "https://hetmanrecovery.com/download.htm".to_string(),
        "CPUID.HWMonitor" => "https://www.cpuid.com/softwares/hwmonitor.html".to_string(),
        "Inkscape.Inkscape" => "https://inkscape.org/release/".to_string(),
        "Instagram.Instagram" => "https://apps.microsoft.com/detail/9nblggh5l9xt".to_string(),
        "IrfanSkiljan.IrfanView" => "https://www.irfanview.com/".to_string(),
        "AppWork.JDownloader" => "https://jdownloader.org/jdownloader2".to_string(),
        "KDE.Kate" => "https://kate-editor.org/get-it/".to_string(),
        "KDE.Kdenlive" => "https://kdenlive.org/en/download/".to_string(),
        "DominikReichl.KeePass" => "https://keepass.info/download.html".to_string(),
        "KiCad.KiCad" => "https://www.kicad.org/download/windows/".to_string(),
        "KDE.Krita" => "https://krita.org/en/download/".to_string(),
        "TheDocumentFoundation.LibreOffice" => "https://www.libreoffice.org/download/download-libreoffice/".to_string(),
        "LMMS.LMMS" => "https://lmms.io/download".to_string(),
        "LMStudio.LMStudio" => "https://lmstudio.ai/download".to_string(),
        "MEGOGO.MEGOGO" => "https://megogo.net/".to_string(),
        "Microsoft.PowerToys" => "https://learn.microsoft.com/en-us/windows/powertoys/install".to_string(),
        "Microsoft.VisualStudio.2022.Community" => "https://visualstudio.microsoft.com/downloads/".to_string(),
        "Mixxx.Mixxx" => "https://mixxx.org/download/".to_string(),
        "MartinPesch.mp3DirectCut" => "https://mpesch3.de/".to_string(),
        "StevenMayall.MusicBee" => "https://getmusicbee.com/downloads/".to_string(),
        "Netflix.Netflix" => "https://apps.microsoft.com/detail/9wzdncrfj3tj".to_string(),
        "Nextcloud.NextcloudDesktop" => "https://nextcloud.com/install/#install-clients".to_string(),
        "Notepad++.Notepad++" => "https://notepad-plus-plus.org/downloads/".to_string(),
        "Nvidia.GeForceNow" => "https://www.nvidia.com/en-us/geforce-now/download/".to_string(),
        "OBSProject.OBSStudio" => "https://obsproject.com/download".to_string(),
        "OCBASE.OCCT" => "https://www.ocbase.com/download".to_string(),
        "KDE.Okular" => "https://okular.kde.org/download/".to_string(),
        "Ollama.Ollama" => "https://ollama.com/download".to_string(),
        "ONLYOFFICE.DesktopEditors" => "https://www.onlyoffice.com/desktop.aspx".to_string(),
        "OpenClaw.OpenClawCompanion" => "https://openclaw.ai/".to_string(),
        "Opera.Opera" => "https://www.opera.com/download".to_string(),
        "Opera.OperaGX" => "https://www.opera.com/gx".to_string(),
        "dotPDN.PaintDotNet" => "https://getpaint.net/download.html".to_string(),
        "Perplexity.Perplexity" => "https://www.perplexity.ai/".to_string(),
        "Picsart.Picsart" => "https://apps.microsoft.com/detail/9nblggh1j2l0".to_string(),
        "Pinterest.Pinterest" => "https://apps.microsoft.com/detail/9wzdncrfj3z6".to_string(),
        "RedHat.Podman-Desktop" => "https://podman-desktop.io/downloads".to_string(),
        "Daum.PotPlayer" => "https://potplayer.daum.net/".to_string(),
        "Amazon.PrimeVideo" => "https://apps.microsoft.com/detail/9p60xg2mvwvq".to_string(),
        "ProtonTechnologies.ProtonVPN" => "https://protonvpn.com/download".to_string(),
        "Qalculate.Qalculate" => "https://qalculate.github.io/downloads.html".to_string(),
        "qBittorrent.qBittorrent" => "https://www.qbittorrent.org/download.php".to_string(),
        "Qmmp.Qmmp" => "https://qmmp.ylsoftware.com/downloads.php".to_string(),
        "RawTherapee.RawTherapee" => "https://rawtherapee.com/downloads/".to_string(),
        "Cockos.REAPER" => "https://www.reaper.fm/download.php".to_string(),
        "reMarkable.reMarkable" => "https://remarkable.com/using-remarkable/apps/desktop".to_string(),
        "Rescuezilla.Rescuezilla" => "https://rescuezilla.com/download".to_string(),
        "VSRevoGroup.RevoUninstaller" => "https://www.revouninstaller.com/revo-uninstaller-free-download/".to_string(),
        "Roblox.Roblox" => "https://www.roblox.com/download".to_string(),
        "Rufus.Rufus" => "https://rufus.ie/".to_string(),
        "Meltytech.Shotcut" => "https://shotcut.org/download/".to_string(),
        "PaulPacifico.ShutterEncoder" => "https://www.shutterencoder.com/en/".to_string(),
        "OpenWhisperSystems.Signal" => "https://signal.org/download/".to_string(),
        "Smarty.Uninstaller" => "https://www.smartyuninstaller.com/download.html".to_string(),
        "Snap.Snapchat" => "https://apps.microsoft.com/detail/9p8wbgv962k4".to_string(),
        "Spotify.Spotify" => "https://www.spotify.com/download/".to_string(),
        "Valve.Steam" => "https://store.steampowered.com/about/".to_string(),
        "Streamlabs.StreamlabsDesktop" => "https://streamlabs.com/".to_string(),
        "SumatraPDF.SumatraPDF" => "https://www.sumatrapdfreader.org/download-free-pdf-viewer".to_string(),
        "Telegram.TelegramDesktop" => "https://desktop.telegram.org/".to_string(),
        "Mozilla.Thunderbird" => "https://www.thunderbird.net/".to_string(),
        "TIDAL.TIDAL" => "https://tidal.com/download".to_string(),
        "TikTok.TikTok" => "https://apps.microsoft.com/detail/9nh2gph4jzs4".to_string(),
        "Transmission.Transmission" => "https://transmissionbt.com/download".to_string(),
        "Twitch.Twitch" => "https://www.twitch.tv/downloads".to_string(),
        "Meta.VCRedistAll" => "https://learn.microsoft.com/en-US/cpp/windows/latest-supported-vc-redist".to_string(),
        "ventoy.Ventoy" => "https://www.ventoy.net/en/download.html".to_string(),
        "Oracle.VirtualBox" => "https://www.virtualbox.org/wiki/Downloads".to_string(),
        "AtomixProductions.VirtualDJ" => "https://www.virtualdj.com/download/".to_string(),
        "VideoLAN.VLC" => "https://www.videolan.org/vlc/".to_string(),
        "VivaldiTechnologies.Vivaldi" => "https://vivaldi.com/download/".to_string(),
        "VSCodium.VSCodium" => "https://vscodium.com/".to_string(),
        "WhatsApp.WhatsApp" => "https://www.whatsapp.com/download".to_string(),
        "RamenSoftware.Windhawk" => "https://windhawk.net/".to_string(),
        "WireGuard.WireGuard" => "https://www.wireguard.com/install/".to_string(),
        "WiseCleaner.WiseCare365" => "https://www.wisecleaner.com/wise-care-365.html".to_string(),
        "NoWinget.Wix" => "https://www.wix.com/".to_string(),
        "XnSoft.XnViewMP" => "https://www.xnview.com/en/xnviewmp/".to_string(),
        "ZenBrowser.Zen" => "https://zen-browser.app/download".to_string(),
        "Zoom.Zoom" => "https://zoom.us/download".to_string(),
        "NoWinget.ActivePresenter" => "https://atomisystems.com/activepresenter/free-edition/".to_string(),
        "axpnet.AeroFTP" => "https://www.aeroftp.app/".to_string(),
        "FinalWire.AIDA64.Extreme" => "https://www.aida64.com/downloads".to_string(),
        "NoWinget.AnimatedWallpaperMaker" => "https://www.desktoppaints.com/about.php?product=wp_maker".to_string(),
        "AOMEI.Backupper.Standard" => "https://www.aomeitech.com/ab/aomei-backupper-standard.html".to_string(),
        "AOMEI.PartitionAssistant" => "https://www.aomeitech.com/download.html".to_string(),
        "NoWinget.Automize" => "https://www.hiteksoftware.com/".to_string(),
        "TechSmith.Camtasia" => "https://www.techsmith.com/download/camtasia/".to_string(),
        "DOSBox.DOSBox" => "https://www.dosbox.com/download.php".to_string(),
        "NoWinget.Doxillion" => "https://www.nchsoftware.com/documentconvert/index.html".to_string(),
        "Dropbox.Dropbox" => "https://www.dropbox.com/download".to_string(),
        "EaseUS.PartitionMaster" => "https://www.easeus.com/partition-manager/epm-free.html".to_string(),
        "EaseUS.TodoBackup" => "https://www.easeus.com/backup-software/tb-free.html".to_string(),
        "NoWinget.FileConverter" => "https://file-converter.org/".to_string(),
        "TimKosse.FileZilla.Client" => "https://filezilla-project.org/download.php?type=client".to_string(),
        "NoWinget.FormatFactory" => "http://www.pcfreetime.com/formatfactory/index.php".to_string(),
        "NoWinget.freac" => "https://www.freac.org/downloads-mainmenu-33".to_string(),
        "Google.GoogleDrive" => "https://www.google.com/drive/download/".to_string(),
        "HandBrake.HandBrake" => "https://handbrake.fr/downloads.php".to_string(),
        "Mailbird.Mailbird" => "https://www.getmailbird.com/download/".to_string(),
        "MediaHuman.AudioConverter" => "https://www.mediahuman.com/audio-converter/".to_string(),
        "Mega.MEGASync" => "https://mega.io/desktop".to_string(),
        "GNU.MidnightCommander" => "https://midnight-commander.org/".to_string(),
        "RustDesk.RustDesk" => "https://rustdesk.com/".to_string(),
        "NoWinget.StrongRecovery" => "https://www.strongrecovery.com/strongrecovery-free.php".to_string(),
        "Ritlabs.TheBat" => "https://www.ritlabs.com/en/products/thebat/".to_string(),
        "Microsoft.BingWallpaper" => "https://www.microsoft.com/en-us/bing/bing-wallpaper".to_string(),
        "NoWinget.4KVideoDownloader" => "https://www.4kdownload.com/products/videodownloader".to_string(),
        "Klocman.BulkCrapUninstaller" => "https://www.bcuninstaller.com/".to_string(),
        "TGRMNSoftware.BulkRenameUtility" => "https://www.bulkrenameutility.co.uk/".to_string(),
        "darktable.darktable" => "https://www.darktable.org/install/".to_string(),
        "NoWinget.EaseUSVideoDownloader" => "https://www.easeus.com/video-downloader/".to_string(),
        "Fastfetch-cli.Fastfetch" => "https://github.com/fastfetch-cli/fastfetch".to_string(),
        "FreeTubeApp.FreeTube" => "https://freetubeapp.io/".to_string(),
        "GSmartControl.GSmartControl" => "https://gsmartcontrol.shurizzle.dev/".to_string(),
        "ImputNet.Helium" => "https://helium.computer/".to_string(),
        "NoWinget.iTubeGo" => "https://www.itubego.com/youtube-downloader/".to_string(),
        "NoWinget.JavaOpenJDK" => "https://adoptium.net/temurin/releases/".to_string(),
        "NoWinget.LockHunter" => "https://lockhunter.com/download.htm".to_string(),
        "nomacs.nomacs" => "https://nomacs.org/".to_string(),
        "CalcProgrammer1.OpenRGB" => "https://openrgb.org/".to_string(),
        "Pidgin.Pidgin" => "https://pidgin.im/download/windows/".to_string(),
        "PuTTY.PuTTY" => "https://www.putty.org/".to_string(),
        "Rainmeter.Rainmeter" => "https://www.rainmeter.net/".to_string(),
        "Genymobile.scrcpy" => "https://github.com/Genymobile/scrcpy".to_string(),
        "NoWinget.Sefirah" => "https://apps.microsoft.com/detail/9pjv6d1jpg0h".to_string(),
        "ShareX.ShareX" => "https://getsharex.com/".to_string(),
        "NoWinget.SnappyDriverInstaller" => "https://sdi-tool.org/".to_string(),
        "KRTirtho.Spotube" => "https://spotube.krtirtho.dev/".to_string(),
        "NoWinget.Tenacity" => "https://tenacityaudio.org/".to_string(),
        "zhongyang219.TrafficMonitor" => "https://github.com/zhongyang219/TrafficMonitor".to_string(),
        _ => format!("https://www.google.com/search?q=download+{}", pkg_name.replace(" ", "+")),
    };

    let _ = Command::new("cmd")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["/C", "start", "", &url])
    .status();

    false
}

fn run_updater() {
    let _ = Command::new("winget").creation_flags(CREATE_NO_WINDOW).args(["source", "update"]).status();
    let _ = Command::new("winget")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["upgrade", "--all", "--silent", "--accept-package-agreements", "--accept-source-agreements", "--include-unknown"])
    .status();
}

fn change_default_browser() {
    let _ = Command::new("powershell")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["-NoProfile", "-Command", "Start-Process ms-settings:defaultapps"])
    .status();
}

fn run_tweak(action: &str, enable: bool) {
    let ps_script = match (action, enable) {
        ("visuals", false) => r#"
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects' -Name 'VisualFXSetting' -Value 2 -Force -ErrorAction SilentlyContinue
        if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' -Force | Out-Null }
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' -Name 'EnableTransparency' -Value 0 -Force -ErrorAction SilentlyContinue

        Set-ItemProperty -Path 'HKCU:\Control Panel\Desktop\WindowMetrics' -Name 'MinAnimate' -Value '0' -Force -ErrorAction SilentlyContinue

        $advPath = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced'
        if (!(Test-Path $advPath)) { New-Item -Path $advPath -Force | Out-Null }
        Set-ItemProperty -Path $advPath -Name 'TaskbarAnimations' -Value 0 -Force -ErrorAction SilentlyContinue

        if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\DWM')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\DWM' -Force | Out-Null }
        Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\DWM' -Name 'Animations' -Value 0 -Force -ErrorAction SilentlyContinue

        if (-not ('Win32.SPI' -as [type])) {
            Add-Type -TypeDefinition 'namespace Win32 { using System; using System.Runtime.InteropServices; public class SPI { [DllImport("user32.dll")] public static extern bool SystemParametersInfo(uint a, uint b, IntPtr c, uint d); } }' -ErrorAction SilentlyContinue
    }
    [Win32.SPI]::SystemParametersInfo(0x1043, 0, [IntPtr]0, 3) | Out-Null
    [Win32.SPI]::SystemParametersInfo(0x103F, 0, [IntPtr]0, 3) | Out-Null
    "#,
    ("visuals", true) => r#"
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects' -Name 'VisualFXSetting' -Value 0 -Force -ErrorAction SilentlyContinue
    if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' -Name 'EnableTransparency' -Value 1 -Force -ErrorAction SilentlyContinue

    Set-ItemProperty -Path 'HKCU:\Control Panel\Desktop\WindowMetrics' -Name 'MinAnimate' -Value '1' -Force -ErrorAction SilentlyContinue

    $advPath = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced'
    if (!(Test-Path $advPath)) { New-Item -Path $advPath -Force | Out-Null }
    Set-ItemProperty -Path $advPath -Name 'TaskbarAnimations' -Value 1 -Force -ErrorAction SilentlyContinue

    if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\DWM')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\DWM' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\DWM' -Name 'Animations' -Value 1 -Force -ErrorAction SilentlyContinue

    if (-not ('Win32.SPI' -as [type])) {
        Add-Type -TypeDefinition 'namespace Win32 { using System; using System.Runtime.InteropServices; public class SPI { [DllImport("user32.dll")] public static extern bool SystemParametersInfo(uint a, uint b, IntPtr c, uint d); } }' -ErrorAction SilentlyContinue
    }
    [Win32.SPI]::SystemParametersInfo(0x1043, 0, [IntPtr]1, 3) | Out-Null
    [Win32.SPI]::SystemParametersInfo(0x103F, 0, [IntPtr]1, 3) | Out-Null
    "#,

    ("services", false) => r#"
    $svcs = @('SysMain', 'WerSvc', 'WMPNetworkSvc', 'TabletInputService', 'RemoteRegistry', 'RemoteAccess', 'SharedAccess', 'Netlogon', 'TapiSrv', 'wisvc', 'WalletService', 'MapsBroker', 'CscService', 'seclogon', 'DPS', 'WdiSystemHost', 'WdiServiceHost', 'CertPropSvc', 'SSDPSRV', 'upnphost', 'NetTcpPortSharing', 'vmicvmsession', 'LanmanWorkstation', 'EventSystem', 'fhsvc', 'WebClient', 'iphlpsvc', 'lmhosts', 'WpcSvc', 'workfolderssvc', 'pla', 'wscsvc', 'ncdauto', 'COMSysApp', 'NcaSvc', 'LanmanServer')
    foreach ($s in $svcs) { Stop-Service -Name $s -Force -ErrorAction SilentlyContinue; Set-Service -Name $s -StartupType Disabled -ErrorAction SilentlyContinue }

    if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\CDP')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\CDP' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\CDP' -Name 'CdpSessionUserAuthzPolicy' -Value 0 -Force -ErrorAction SilentlyContinue
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\CDP' -Name 'NearShareChannelUserAuthzPolicy' -Value 0 -Force -ErrorAction SilentlyContinue

    if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Privacy')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Privacy' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Privacy' -Name 'TailoredExperiencesWithDiagnosticDataEnabled' -Value 0 -Force -ErrorAction SilentlyContinue

    if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo' -Name 'Enabled' -Value 0 -Force -ErrorAction SilentlyContinue
    if (!(Test-Path 'HKCU:\Control Panel\International\User Profile')) { New-Item -Path 'HKCU:\Control Panel\International\User Profile' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Control Panel\International\User Profile' -Name 'HttpAcceptLanguageOptOut' -Value 1 -Force -ErrorAction SilentlyContinue
    "#,
    ("services", true) => r#"
    $svcs = @('SysMain', 'WerSvc', 'WMPNetworkSvc', 'TabletInputService', 'RemoteRegistry', 'RemoteAccess', 'SharedAccess', 'Netlogon', 'TapiSrv', 'wisvc', 'WalletService', 'MapsBroker', 'CscService', 'seclogon', 'DPS', 'WdiSystemHost', 'WdiServiceHost', 'CertPropSvc', 'SSDPSRV', 'upnphost', 'NetTcpPortSharing', 'vmicvmsession', 'LanmanWorkstation', 'EventSystem', 'fhsvc', 'WebClient', 'iphlpsvc', 'lmhosts', 'WpcSvc', 'workfolderssvc', 'pla', 'wscsvc', 'ncdauto', 'COMSysApp', 'NcaSvc', 'LanmanServer')
    foreach ($s in $svcs) { Set-Service -Name $s -StartupType Manual -ErrorAction SilentlyContinue; Start-Service -Name $s -ErrorAction SilentlyContinue }

    if (Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\CDP') {
        Remove-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\CDP' -Name 'CdpSessionUserAuthzPolicy' -ErrorAction SilentlyContinue
        Remove-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\CDP' -Name 'NearShareChannelUserAuthzPolicy' -ErrorAction SilentlyContinue
    }

    if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Privacy')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Privacy' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Privacy' -Name 'TailoredExperiencesWithDiagnosticDataEnabled' -Value 1 -Force -ErrorAction SilentlyContinue

    if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo' -Name 'Enabled' -Value 1 -Force -ErrorAction SilentlyContinue
    if (!(Test-Path 'HKCU:\Control Panel\International\User Profile')) { New-Item -Path 'HKCU:\Control Panel\International\User Profile' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Control Panel\International\User Profile' -Name 'HttpAcceptLanguageOptOut' -Value 0 -Force -ErrorAction SilentlyContinue
    "#,

    ("wsearch", false) => "Stop-Service -Name 'WSearch' -Force -ErrorAction SilentlyContinue; Set-Service -Name 'WSearch' -StartupType Disabled -ErrorAction SilentlyContinue",
    ("wsearch", true) => "Set-Service -Name 'WSearch' -StartupType Automatic -ErrorAction SilentlyContinue; Start-Service -Name 'WSearch' -ErrorAction SilentlyContinue",

    ("spooler", false) => r#"
    Stop-Service -Name 'Spooler' -Force -ErrorAction SilentlyContinue
    Set-Service -Name 'Spooler' -StartupType Disabled -ErrorAction SilentlyContinue
    if (!(Test-Path 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Windows')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Windows' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Windows' -Name 'LegacyDefaultPrinterMode' -Value 1 -Type DWord -Force -ErrorAction SilentlyContinue
    "#,
    ("spooler", true) => r#"
    Set-Service -Name 'Spooler' -StartupType Automatic -ErrorAction SilentlyContinue
    Start-Service -Name 'Spooler' -ErrorAction SilentlyContinue
    if (!(Test-Path 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Windows')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Windows' -Force | Out-Null }
    Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Windows' -Name 'LegacyDefaultPrinterMode' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
    "#,

    ("wia", false) => "Stop-Service -Name 'stisvc' -Force -ErrorAction SilentlyContinue; Set-Service -Name 'stisvc' -StartupType Disabled -ErrorAction SilentlyContinue",
    ("wia", true) => "Set-Service -Name 'stisvc' -StartupType Automatic -ErrorAction SilentlyContinue; Start-Service -Name 'stisvc' -ErrorAction SilentlyContinue",
    ("bluetooth", false) => "Stop-Service -Name 'bthserv' -Force -ErrorAction SilentlyContinue; Set-Service -Name 'bthserv' -StartupType Disabled -ErrorAction SilentlyContinue",
    ("bluetooth", true) => "Set-Service -Name 'bthserv' -StartupType Automatic -ErrorAction SilentlyContinue; Start-Service -Name 'bthserv' -ErrorAction SilentlyContinue",
    ("wifi", false) => "Stop-Service -Name 'WlanSvc' -Force -ErrorAction SilentlyContinue; Set-Service -Name 'WlanSvc' -StartupType Disabled -ErrorAction SilentlyContinue",
    ("wifi", true) => "Set-Service -Name 'WlanSvc' -StartupType Automatic -ErrorAction SilentlyContinue; Start-Service -Name 'WlanSvc' -ErrorAction SilentlyContinue",

    ("updates", false) => r#"
    $pauseDate = (Get-Date).AddDays(180).ToString("yyyy-MM-ddTHH:mm:ssZ")
    $path = "HKLM:\SOFTWARE\Microsoft\WindowsUpdate\UX\Settings"
    if (-not (Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
    Set-ItemProperty -Path $path -Name "PauseFeatureUpdatesStartTime" -Value (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ") -Force
    Set-ItemProperty -Path $path -Name "PauseFeatureUpdatesEndTime" -Value $pauseDate -Force
    Set-ItemProperty -Path $path -Name "PauseQualityUpdatesStartTime" -Value (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssZ") -Force
    Set-ItemProperty -Path $path -Name "PauseQualityUpdatesEndTime" -Value $pauseDate -Force
    Set-ItemProperty -Path $path -Name "PauseUpdatesExpiryTime" -Value $pauseDate -Force
    "#,
    ("updates", true) => r#"
    $path = "HKLM:\SOFTWARE\Microsoft\WindowsUpdate\UX\Settings"
    Remove-ItemProperty -Path $path -Name "PauseFeatureUpdatesStartTime" -ErrorAction SilentlyContinue
    Remove-ItemProperty -Path $path -Name "PauseFeatureUpdatesEndTime" -ErrorAction SilentlyContinue
    Remove-ItemProperty -Path $path -Name "PauseQualityUpdatesStartTime" -ErrorAction SilentlyContinue
    Remove-ItemProperty -Path $path -Name "PauseQualityUpdatesEndTime" -ErrorAction SilentlyContinue
    Remove-ItemProperty -Path $path -Name "PauseUpdatesExpiryTime" -ErrorAction SilentlyContinue
    "#,

    ("hibernation", false) => "powercfg.exe /hibernate off",
    ("hibernation", true) => "powercfg.exe /hibernate on",

    ("bitlocker", false) => r#"
    Disable-BitLocker -MountPoint $env:SystemDrive -ErrorAction SilentlyContinue
    manage-bde -off $env:SystemDrive -ErrorAction SilentlyContinue
    "#,
    ("bitlocker", true) => "Resume-BitLocker -MountPoint $env:SystemDrive -ErrorAction SilentlyContinue",

    ("uac", false) => r#"
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" -Name "EnableLUA" -Value 0 -Force -ErrorAction SilentlyContinue
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" -Name "ConsentPromptBehaviorAdmin" -Value 0 -Force -ErrorAction SilentlyContinue
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" -Name "PromptOnSecureDesktop" -Value 0 -Force -ErrorAction SilentlyContinue
    "#,
    ("uac", true) => r#"
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" -Name "EnableLUA" -Value 1 -Force -ErrorAction SilentlyContinue
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" -Name "ConsentPromptBehaviorAdmin" -Value 5 -Force -ErrorAction SilentlyContinue
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" -Name "PromptOnSecureDesktop" -Value 1 -Force -ErrorAction SilentlyContinue
    "#,

    _ => ""
    };

    if !ps_script.is_empty() {
        let _ = Command::new("powershell")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["-NoProfile", "-Command", ps_script])
        .status();
    }
}

fn run_cleanup_tasks(tasks: Vec<String>, status: &Arc<Mutex<String>>, ctx: &egui::Context, lang_is_pl: bool, cancel_flag: &Arc<AtomicBool>) {
    let update_status = |msg: &str| {
        *status.lock().unwrap() = msg.to_string();
        ctx.request_repaint();
    };

    for task_id in tasks {
        if cancel_flag.load(Ordering::SeqCst) {
            update_status(if lang_is_pl { "🛑 Proces anulowany!" } else { "🛑 Process canceled!" });
            return;
        }

        match task_id.as_str() {
            "standard" => {
                update_status(if lang_is_pl { "Wykonywanie: Standardowe Czyszczenie..." } else { "Running: Standard Cleanup..." });
                run_optimizer();
            }
            "optymalizacja_explorer" => {
                update_status(if lang_is_pl { "Optymalizacja: Eksplorator plików..." } else { "Optimization: File Explorer..." });
                let ps_script = r#"
                Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'LaunchTo' -Value 1 -Force
                Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'HideFileExt' -Value 0 -Force
                $clsids = @(
                    '{f874310e-b6b7-47dc-bc84-b9e6b38f5903}',
                    '{e88865ea-0e1c-4e20-9aa6-edcd0212c87c}',
                    '{031E4825-7B94-4dc3-B131-E946B44C8DD5}'
                    )
                    foreach ($clsid in $clsids) {
                        $key = "HKCU:\Software\Classes\CLSID\$clsid"
                        if (-not (Test-Path $key)) { New-Item -Path $key -Force | Out-Null }
                        Set-ItemProperty -Path $key -Name 'System.IsPinnedToNameSpaceTree' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            }
            $hubKey = 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer'
            Set-ItemProperty -Path $hubKey -Name 'HubMode' -Value 1 -Type DWord -Force -ErrorAction SilentlyContinue

            $zipKey = 'Registry::HKEY_CLASSES_ROOT\.zip\CompressedFolder\ShellNew'
            if (Test-Path $zipKey) {
                Remove-Item -Path $zipKey -Recurse -Force -ErrorAction SilentlyContinue
            }

            $bmpKey = 'Registry::HKEY_CLASSES_ROOT\.bmp\ShellNew'
            if (Test-Path $bmpKey) {
                Remove-Item -Path $bmpKey -Recurse -Force -ErrorAction SilentlyContinue
            }
            "#;
            let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "start_menu" => {
                update_status(if lang_is_pl { "Optymalizacja: Menu Start..." } else { "Optimization: Start Menu..." });
                let ps_script = r#"
                $advPath = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced'
                if (!(Test-Path $advPath)) { New-Item -Path $advPath -Force | Out-Null }
                Set-ItemProperty -Path $advPath -Name 'Start_TrackDocs' -Value 0 -Force -ErrorAction SilentlyContinue
                Set-ItemProperty -Path $advPath -Name 'Start_EnableAccountNotifications' -Value 0 -Force -ErrorAction SilentlyContinue
                Set-ItemProperty -Path $advPath -Name 'Start_AccountNotifications' -Value 0 -Force -ErrorAction SilentlyContinue
                Set-ItemProperty -Path $advPath -Name 'Start_Layout' -Value 1 -Force -ErrorAction SilentlyContinue
                Set-ItemProperty -Path $advPath -Name 'Start_TrackProgs' -Value 0 -Force -ErrorAction SilentlyContinue
                Set-ItemProperty -Path $advPath -Name 'Start_IrisRecommendations' -Value 0 -Force -ErrorAction SilentlyContinue
                $polPath = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Explorer'
                if (!(Test-Path $polPath)) { New-Item -Path $polPath -Force | Out-Null }
                Set-ItemProperty -Path $polPath -Name 'HideAccountNotifications' -Value 1 -Force -ErrorAction SilentlyContinue
                Set-ItemProperty -Path $polPath -Name 'HideRecentlyAddedApps' -Value 1 -Force -ErrorAction SilentlyContinue
                Set-ItemProperty -Path $polPath -Name 'ShowOrHideMostUsedApps' -Value 2 -Force -ErrorAction SilentlyContinue
                "#;
                let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "classic_context_menu" => {
                update_status(if lang_is_pl { "Przełączanie: Klasyczne menu kontekstowe..." } else { "Toggling: Classic context menu..." });
                let ps_script = r#"
                $path = 'HKCU\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}'
                reg query $path 2>$null
                if ($LASTEXITCODE -eq 0) {
                    reg delete $path /f
            } else {
                reg add "$path\InprocServer32" /f /ve
            }
            "#;
            let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "sfc" => {
                update_status(if lang_is_pl { "Wykonywanie: SFC /scannow (To potrwa chwilę)..." } else { "Running: SFC /scannow (This takes a while)..." });
                let _ = Command::new("sfc").args(["/scannow"]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "dism" => {
                update_status(if lang_is_pl { "Wykonywanie: DISM /RestoreHealth (To potrwa chwilę)..." } else { "Running: DISM /RestoreHealth (This takes a while)..." });
                let _ = Command::new("dism").args(["/Online", "/Cleanup-Image", "/RestoreHealth"]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "chkdsk" => {
                update_status(if lang_is_pl { "Planowanie: CHKDSK (Potwierdź Y w czarnym oknie)..." } else { "Scheduling: CHKDSK (Confirm Y in black window)..." });
                let _ = Command::new("cmd").args(["/C", "start", "cmd", "/C", "echo Uruchamianie CHKDSK. Jeśli zapyta, wpisz T lub Y aby zaplanować. & chkdsk C: /f & pause"]).status();
            }
            "trim" => {
                update_status(if lang_is_pl { "Wykonywanie: TRIM (Optymalizacja dysku C:)..." } else { "Running: TRIM (Drive C: optimization)..." });
                let _ = Command::new("powershell").args(["-NoProfile", "-Command", "Optimize-Volume -DriveLetter C -ReTrim"]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "remove_apps" => {
                update_status(if lang_is_pl { "Usuwanie: Wbudowany bloatware..." } else { "Removing: Built-in apps..." });
                run_bloatware_remover();
            }
            "remove_copilot" => {
                update_status(if lang_is_pl { "Usuwanie: AI Copilot..." } else { "Removing: AI Copilot..." });
                run_copilot_remover();
            }
            "remove_calc" => {
                update_status(if lang_is_pl { "Usuwanie: Kalkulator..." } else { "Removing: Calculator..." });
                let ps_script = "Get-AppxPackage *windowscalculator* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue";
                let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "remove_edge" => {
                update_status(if lang_is_pl { "Usuwanie: Microsoft Edge..." } else { "Removing: Microsoft Edge..." });
                let ps_script = r#"
                winget uninstall --id Microsoft.Edge -e --silent --accept-source-agreements
                Get-AppxPackage *MicrosoftEdge* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
                "#;
                let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "remove_notepad" => {
                update_status(if lang_is_pl { "Usuwanie: Notatnik..." } else { "Removing: Notepad..." });
                let ps_script = "Get-AppxPackage *Microsoft.WindowsNotepad* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue";
                let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "remove_xbox_apps" => {
                update_status(if lang_is_pl { "Usuwanie i wyłączanie: Usługi Xbox i GameBar..." } else { "Removing and disabling: Xbox & GameBar..." });
                let ps_script = r#"
                Stop-Process -Name "GameBarFTServer", "GamebarPresenceWriter", "bcastdvr", "XboxApp", "XboxPcAppFT", "XboxGamingOverlay", "xboxmode" -Force -ErrorAction SilentlyContinue
                Get-AppxPackage *Xbox* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
                Get-AppxPackage *XboxGamingOverlay* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
                Get-AppxPackage *XboxIdentityProvider* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
                Get-AppxPackage *XboxSpeechToTextOverlay* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
                Get-AppxPackage *xboxmode* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
                $svcs = @('XblAuthManager', 'XboxNetApiSvc', 'XboxGipSvc', 'xbgm', 'BcastDVRUserService', 'xboxmode')
                foreach ($s in $svcs) {
                    Stop-Service -Name $s -Force -ErrorAction SilentlyContinue
                    Set-Service -Name $s -StartupType Disabled -ErrorAction SilentlyContinue
            }
            if (!(Test-Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\GameDVR')) { New-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\GameDVR' -Force | Out-Null }
            Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\GameDVR' -Name 'AppCaptureEnabled' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            if (!(Test-Path 'HKCU:\System\GameConfigStore')) { New-Item -Path 'HKCU:\System\GameConfigStore' -Force | Out-Null }
            Set-ItemProperty -Path 'HKCU:\System\GameConfigStore' -Name 'GameDVR_Enabled' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path 'HKCU:\System\GameConfigStore' -Name 'GameDVR_FSEBehaviorMode' -Value 2 -Type DWord -Force -ErrorAction SilentlyContinue
            if (!(Test-Path 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\GameDVR')) { New-Item -Path 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\GameDVR' -Force | Out-Null }
            Set-ItemProperty -Path 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\GameDVR' -Name 'AllowGameDVR' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            if (!(Test-Path 'HKCU:\Software\Microsoft\GameBar')) { New-Item -Path 'HKCU:\Software\Microsoft\GameBar' -Force | Out-Null }
            Set-ItemProperty -Path 'HKCU:\Software\Microsoft\GameBar' -Name 'UseNexusForGameBarEnabled' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path 'HKCU:\Software\Microsoft\GameBar' -Name 'ShowStartupPanel' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path 'HKCU:\Software\Microsoft\GameBar' -Name 'AllowAutoGameMode' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path 'HKCU:\Software\Microsoft\GameBar' -Name 'UseViewAndMenuAsGuide' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path 'HKCU:\Software\Microsoft\GameBar' -Name 'GuideButtonInApps' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            if (!(Test-Path 'HKCU:\Software\Microsoft\XboxApp')) { New-Item -Path 'HKCU:\Software\Microsoft\XboxApp' -Force | Out-Null }
            Set-ItemProperty -Path 'HKCU:\Software\Microsoft\XboxApp' -Name 'GuideButtonInApps' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            Set-ItemProperty -Path 'HKCU:\Software\Microsoft\XboxApp' -Name 'UseViewAndMenuAsGuide' -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            Get-ScheduledTask -TaskPath "\Microsoft\XblGameSave\" -ErrorAction SilentlyContinue | Disable-ScheduledTask -ErrorAction SilentlyContinue
            "#;
            let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "remove_widgets" => {
                update_status(if lang_is_pl { "Usuwanie: Widżety..." } else { "Removing: Widgets..." });
                let ps_script = r#"
                Get-AppxPackage *MicrosoftWindows.Client.WebExperience* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
                $registryPath = 'HKLM:\SOFTWARE\Policies\Microsoft\Dsh'
                if (-not (Test-Path $registryPath)) { New-Item -Path $registryPath -Force | Out-Null }
                Set-ItemProperty -Path $registryPath -Name 'AllowNewsAndInterests' -Value 0 -Force
                "#;
                let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "remove_yourphone" => {
                update_status(if lang_is_pl { "Usuwanie: Łącze z telefonem..." } else { "Removing: Phone Link..." });
                let ps_script = "Get-AppxPackage *Microsoft.YourPhone* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue";
                let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "remove_rdp" => {
                update_status(if lang_is_pl { "Usuwanie i blokowanie: Pulpit Zdalny (RDP)..." } else { "Removing and disabling: Remote Desktop (RDP)..." });
                let ps_script = r#"
                Get-AppxPackage *Microsoft.RemoteDesktop* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
                Set-ItemProperty -Path 'HKLM:\System\CurrentControlSet\Control\Terminal Server' -Name 'fDenyTSConnections' -Value 1 -Force -ErrorAction SilentlyContinue
                Stop-Service -Name TermService -Force -ErrorAction SilentlyContinue
                Set-Service -Name TermService -StartupType Disabled -ErrorAction SilentlyContinue
                "#;
                let _ = Command::new("powershell").args(["-NoProfile", "-Command", ps_script]).creation_flags(CREATE_NO_WINDOW).status();
            }
            "msconfig" => {
                update_status(if lang_is_pl { "Otwieranie: MSConfig..." } else { "Opening: MSConfig..." });
                let _ = Command::new("msconfig").status();
            }
            "resmon" => {
                update_status(if lang_is_pl { "Otwieranie: Resmon..." } else { "Opening: Resmon..." });
                let _ = Command::new("resmon").status();
            }
            "perfmon" => {
                update_status(if lang_is_pl { "Otwieranie: Perfmon..." } else { "Opening: Perfmon..." });
                let _ = Command::new("perfmon").args(["/rel"]).status();
            }
            "mdsched" => {
                update_status(if lang_is_pl { "Otwieranie: Mdsched..." } else { "Opening: Mdsched..." });
                let _ = Command::new("mdsched").status();
            }
            "dns_cloudflare" => {
                update_status(if lang_is_pl { "Zmienianie: Serwery DNS na Cloudflare..." } else { "Changing: DNS servers to Cloudflare..." });
                let ps_script = "Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | Set-DnsClientServerAddress -ServerAddresses '1.1.1.1', '1.0.0.1', '2606:4700:4700::1111', '2606:4700:4700::1001'";
                let _ = Command::new("powershell")
                .args(["-NoProfile", "-Command", ps_script])
                .creation_flags(CREATE_NO_WINDOW)
                .status();
            }
            _ => {}
        }
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}

fn run_bloatware_remover() {
    let ps_script = r#"
    Get-AppxPackage -AllUsers | Where-Object { $_.Name -notmatch 'Store' -and $_.Name -notmatch 'Notepad' -and $_.Name -notmatch 'Xbox' -and $_.Name -notmatch 'calculator' -and $_.NonRemovable -eq $false } | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
    Get-AppxProvisionedPackage -Online | Where-Object { $_.DisplayName -notmatch 'Store' -and $_.DisplayName -notmatch 'Notepad' -and $_.DisplayName -notmatch 'Xbox' -and $_.DisplayName -notmatch 'calculator' } | Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue
    "#;

    let _ = Command::new("powershell")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["-NoProfile", "-Command", ps_script])
    .status();
}

fn run_copilot_remover() {
    let ps_script = r#"
    $registryPath = 'HKCU:\Software\Policies\Microsoft\Windows\WindowsCopilot'
    if (-not (Test-Path $registryPath)) { New-Item -Path $registryPath -Force | Out-Null }
    Set-ItemProperty -Path $registryPath -Name 'TurnOffWindowsCopilot' -Value 1 -Force
    $registryPathHKLM = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot'
    if (-not (Test-Path $registryPathHKLM)) { New-Item -Path $registryPathHKLM -Force | Out-Null }
    Set-ItemProperty -Path $registryPathHKLM -Name 'TurnOffWindowsCopilot' -Value 1 -Force
    Get-AppxPackage *Microsoft.Windows.Ai.Copilot.Provider* -AllUsers | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
    "#;

    let _ = Command::new("powershell")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["-NoProfile", "-Command", ps_script])
    .status();
}

fn run_optimizer() {
    let ps_script = r#"
    $ProcessNames = "chrome", "msedge", "brave", "vivaldi", "opera", "firefox", "explorer", "discord", "Telegram", "WhatsApp", "Twitch"
    foreach ($p in $ProcessNames) { Stop-Process -Name $p -Force -ErrorAction SilentlyContinue }
    Start-Sleep -Seconds 2

    Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "LaunchTo" -Value 1 -Force
    Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer" -Name "ShowRecent" -Value 0 -Force
    Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer" -Name "ShowFrequent" -Value 0 -Force
    Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "Start_TrackDocs" -Value 0 -Force

    if (-not (Test-Path "HKCU:\Software\Microsoft\Clipboard")) { New-Item -Path "HKCU:\Software\Microsoft\Clipboard" -Force | Out-Null }
    Set-ItemProperty -Path "HKCU:\Software\Microsoft\Clipboard" -Name "EnableClipboardHistory" -Value 1 -Force

    Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name "Start_IrisRecommendations" -Value 0 -Force
    if (-not (Test-Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Explorer")) { New-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Explorer" -Force | Out-Null }
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Explorer" -Name "HideRecommendedSection" -Value 1 -Force -ErrorAction SilentlyContinue

    New-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection" -Force -ErrorAction SilentlyContinue | Out-Null
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection" -Name "AllowTelemetry" -Value 0 -Type DWord -ErrorAction SilentlyContinue
    Stop-Service -Name "DiagTrack" -Force -ErrorAction SilentlyContinue
    Set-Service -Name "DiagTrack" -StartupType Disabled -ErrorAction SilentlyContinue

    $StartupApproved = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run"
    if (Test-Path $StartupApproved) {
        Get-ItemProperty $StartupApproved | Select-Object -Property * -ExcludeProperty PSPath, PSParentPath, PSChildName, PSDrive, PSProvider, "SecurityHealth" | ForEach-Object {
        foreach ($prop in $_.psobject.properties.name) {
            Set-ItemProperty -Path $StartupApproved -Name $prop -Value ([byte[]](0x03,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00)) -Force -ErrorAction SilentlyContinue
}
}
}

$RunKey = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Run"
if (Test-Path $RunKey) {
    Get-ItemProperty $RunKey | Select-Object -Property * -ExcludeProperty PSPath, PSParentPath, PSChildName, PSDrive, PSProvider, "SecurityHealth", "OneDrive" | ForEach-Object {
    foreach ($prop in $_.psobject.properties.name) {
        Remove-ItemProperty -Path $RunKey -Name $prop -Force -ErrorAction SilentlyContinue
}
}
}

Clear-DnsClientCache
ipconfig /flushdns | Out-Null

$HistoryPaths = @(
    "$env:APPDATA\Microsoft\Windows\Recent\*",
    "$env:APPDATA\Microsoft\Windows\Recent\AutomaticDestinations\*",
    "$env:LOCALAPPDATA\Microsoft\Windows\Explorer\thumbcache_*.db",
    "C:\Windows\Prefetch\*"
    )
    foreach ($h in $HistoryPaths) { Remove-Item -Path $h -Recurse -Force -ErrorAction SilentlyContinue }

    Remove-Item -Path "$env:USERPROFILE\Desktop\*.lnk" -Force -ErrorAction SilentlyContinue
    Remove-Item -Path "$env:USERPROFILE\Desktop\*.url" -Force -ErrorAction SilentlyContinue
    Remove-Item -Path "$env:PUBLIC\Desktop\*.lnk" -Force -ErrorAction SilentlyContinue
    Remove-Item -Path "$env:PUBLIC\Desktop\*.url" -Force -ErrorAction SilentlyContinue

    $ShaderPaths = @("$env:LOCALAPPDATA\D3DSCache\*", "$env:LOCALAPPDATA\NVIDIA\GLCache\*", "$env:LOCALAPPDATA\AMD\DxCache\*")
    foreach ($s in $ShaderPaths) { Remove-Item -Path $s -Recurse -Force -ErrorAction SilentlyContinue }

    $BrowserPaths = @(
        "$env:LOCALAPPDATA\Google\Chrome\User Data\Default\Cache\*",
        "$env:LOCALAPPDATA\Microsoft\Edge\User Data\Default\Cache\*",
        "$env:LOCALAPPDATA\BraveSoftware\Brave-Browser\User Data\Default\Cache\*",
        "$env:LOCALAPPDATA\Vivaldi\User Data\Default\Cache\*",
        "$env:LOCALAPPDATA\Mozilla\Firefox\Profiles\*\cache2\*",
        "$env:LOCALAPPDATA\Opera Software\Opera Stable\Cache\*",
        "$env:LOCALAPPDATA\Opera Software\Opera GX Stable\Cache\*"
        )
        foreach ($path in $BrowserPaths) { Remove-Item -Path $path -Recurse -Force -ErrorAction SilentlyContinue }

        $AppCachePaths = @(
            "$env:APPDATA\discord\Cache\*",
            "$env:APPDATA\discord\Code Cache\*",
            "$env:APPDATA\Telegram Desktop\tdata\user_data\cache\*",
            "$env:LOCALAPPDATA\Packages\*WhatsApp*\LocalCache\*",
            "$env:LOCALAPPDATA\Packages\*WhatsApp*\LocalState\TempState\*",
            "$env:APPDATA\Twitch\Cache\*",
            "$env:APPDATA\Twitch\Code Cache\*"
            )
            foreach ($path in $AppCachePaths) { Remove-Item -Path $path -Recurse -Force -ErrorAction SilentlyContinue }

            $DumpPaths = @(
                "$env:LOCALAPPDATA\CrashDumps\*",
                "C:\Windows\Minidump\*"
                )
                foreach ($path in $DumpPaths) { Remove-Item -Path $path -Recurse -Force -ErrorAction SilentlyContinue }

                $BloatFolders = @(
                    "$env:ProgramFiles\Internet Explorer",
                    "${env:ProgramFiles(x86)}\Internet Explorer",
                    "$env:ProgramFiles\Windows Media Player",
                    "${env:ProgramFiles(x86)}\Windows Media Player",
                    "$env:ProgramFiles\Windows NT",
                    "${env:ProgramFiles(x86)}\Windows NT",
                    "$env:ProgramFiles\Windows Mail",
                    "${env:ProgramFiles(x86)}\Windows Mail"
                    )
                    foreach ($folder in $BloatFolders) {
                        if (Test-Path $folder) {
                            cmd.exe /c "takeown /f `"$folder`" /a /r /d y" 2>&1 | Out-Null
                            cmd.exe /c "icacls `"$folder`" /grant *S-1-5-32-544:F /t /c /q" 2>&1 | Out-Null
                            Remove-Item -Path $folder -Recurse -Force -ErrorAction SilentlyContinue
}
}

$TargetDirs = @("$env:ProgramFiles", "${env:ProgramFiles(x86)}", "$env:ProgramData")
foreach ($dir in $TargetDirs) {
    if (Test-Path $dir) {
        Get-ChildItem -Path $dir -Recurse -Directory -ErrorAction SilentlyContinue | Where-Object { @(Get-ChildItem -Path $_.FullName -Force -ErrorAction SilentlyContinue).Count -eq 0 } | Remove-Item -Force -ErrorAction SilentlyContinue
}
}

$LangDirs = @("es-ES", "fr-FR", "de-DE", "it-IT", "ru-RU", "zh-CN", "zh-TW", "ja-JP", "ko-KR", "pt-BR", "pt-PT", "nl-NL", "tr-TR", "uk-UA", "cs-CZ")
foreach ($dir in $TargetDirs) {
    if (Test-Path $dir) {
        foreach ($lang in $LangDirs) {
            Get-ChildItem -Path $dir -Recurse -Filter $lang -Directory -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
}
}
}

Get-EventLog -List | ForEach-Object { Clear-EventLog -LogName $_.Log }
Remove-Item -Path "$env:TEMP\*" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path "C:\Windows\Temp\*" -Recurse -Force -ErrorAction SilentlyContinue
Clear-RecycleBin -Confirm:$false -ErrorAction SilentlyContinue

Stop-Service -Name "wuauserv" -Force -ErrorAction SilentlyContinue
Remove-Item -Path "C:\Windows\SoftwareDistribution\Download\*" -Recurse -Force -ErrorAction SilentlyContinue
Start-Service -Name "wuauserv"

dism /online /cleanup-image /startcomponentcleanup /resetbase | Out-Null

Start-Process "explorer.exe"
"#;

let _ = Command::new("powershell")
.creation_flags(CREATE_NO_WINDOW)
.args(["-NoProfile", "-Command", ps_script])
.status();
}

fn restart_explorer() {
    let ps_script = "Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue; Start-Process explorer";
    let _ = Command::new("powershell")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["-NoProfile", "-Command", ps_script])
    .status();
}

fn open_url(url: &str) {
    let _ = Command::new("cmd")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["/C", "start", "", url])
    .status();
}

fn open_apps_folder() {
    let _ = Command::new("explorer")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["shell:appsFolder"])
    .status();
}

fn open_god_mode() {
    let _ = Command::new("explorer")
    .creation_flags(CREATE_NO_WINDOW)
    .args(["shell:::{ED7BA470-8E54-465E-825C-99712043E01C}"])
    .status();
}

fn load_icon() -> egui::IconData {
    let image_data = include_bytes!("../icon.png");
    let image = image::load_from_memory(image_data)
    .expect("Nie mozna zaladowac ikony icon.png. Upewnij sie, ze znajduje sie w glownym folderze projektu.")
    .into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    egui::IconData { rgba, width, height }
}

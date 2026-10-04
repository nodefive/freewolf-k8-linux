//! Internationalization (i18n) module for FREE WOLF K8 Linux Controller

pub const LANGUAGES: &[(&str, &str)] = &[
    ("en", "English"),
    ("pt", "Português"),
    ("es", "Español"),
    ("fr", "Français"),
    ("de", "Deutsch"),
];

pub fn get_mode_name(lang: &str, mode_id: u8, default_name: &'static str) -> &'static str {
    match lang {
        "pt" => match mode_id {
            0 => "Desligado", 1 => "Estático", 2 => "Respiração", 3 => "Moinho de Vento",
            4 => "Fluxo Neon", 5 => "Onda de Luz", 6 => "Luz Fluida", 7 => "Ondulações",
            8 => "Ponto Brilhante", 9 => "Flash Fugaz", 10 => "Sombra Evanescente",
            11 => "Ondas Brilhantes", 12 => "Cores Nobres", 13 => "Efeito Letreiro",
            14 => "Tempestade Giratória", 15 => "Corrida de Cavalos", 16 => "Cintilação Estelar",
            17 => "Cobrinha Retrô", 18 => "Onda Diagonal", 19 => "Onda Senoidal", 20 => "Música",
            _ => default_name,
        },
        "es" => match mode_id {
            0 => "Apagado", 1 => "Estático", 2 => "Respiración", 3 => "Molinete",
            4 => "Flujo Neón", 5 => "Onda de Luz", 6 => "Luz Fluida", 7 => "Gotas Ondulantes",
            8 => "Punto Brillante", 9 => "Flash Fugaz", 10 => "Sombra Fugitiva",
            11 => "Ondas Brillantes", 12 => "Colores Nobles", 13 => "Efecto Marquesina",
            14 => "Tormenta Giratoria", 15 => "Carrera Serpenteante", 16 => "Estrellas Brillantes",
            17 => "Serpiente Retro", 18 => "Onda Diagonal", 19 => "Onda Sinusoidal", 20 => "Música",
            _ => default_name,
        },
        "fr" => match mode_id {
            0 => "Éteint", 1 => "Fixe", 2 => "Respiration", 3 => "Moulin à Vent",
            4 => "Flux Néon", 5 => "Vague de Lumière", 6 => "Lumière Fluide", 7 => "Gouttes d'Eau",
            8 => "Point Brillant", 9 => "Éclair Fugitif", 10 => "Ombre Évanescente",
            11 => "Ondulations Éclatantes", 12 => "Harmonie Noble", 13 => "Effet Chapiteau",
            14 => "Tempête Tournante", 15 => "Course de Chevaux", 16 => "Étoiles Scintillantes",
            17 => "Serpent Rétro", 18 => "Onde Diagonale", 19 => "Onde Sinusoïdale", 20 => "Musique",
            _ => default_name,
        },
        "de" => match mode_id {
            0 => "Aus", 1 => "Statisch", 2 => "Atmung", 3 => "Windmühle",
            4 => "Neon-Strom", 5 => "Lauflicht", 6 => "Fließendes Licht", 7 => "Tropfende Wellen",
            8 => "Leuchtender Punkt", 9 => "Aufblitzen", 10 => "Schatten verblassen",
            11 => "Leuchtende Wellen", 12 => "Harmonische Farben", 13 => "Laufband-Effekt",
            14 => "Wirbelsturm", 15 => "Pferderennen", 16 => "Sternenfunkeln",
            17 => "Retro-Schlange", 18 => "Diagonale Welle", 19 => "Sinuswelle", 20 => "Musik",
            _ => default_name,
        },
        _ => default_name,
    }
}

pub fn t<'a>(lang: &str, key: &'a str) -> &'a str {
    match (lang, key) {
        // App title
        ("pt", "app_title") => "FREEWOLF K8 - Linux",
        ("es", "app_title") => "FREEWOLF K8 - Linux",
        ("fr", "app_title") => "FREEWOLF K8 - Linux",
        ("de", "app_title") => "FREEWOLF K8 - Linux",
        (_, "app_title") => "FREEWOLF K8 - Linux",

        // Tabs
        ("pt", "tab_light") => "Iluminação",
        ("es", "tab_light") => "Iluminación",
        ("fr", "tab_light") => "Éclairage",
        ("de", "tab_light") => "Beleuchtung",
        (_, "tab_light") => "Light",

        ("pt", "tab_macro") => "Macro",
        ("es", "tab_macro") => "Macro",
        ("fr", "tab_macro") => "Macro",
        ("de", "tab_macro") => "Makro",
        (_, "tab_macro") => "Macro",

        ("pt", "tab_help") => "Ajuda",
        ("es", "tab_help") => "Ayuda",
        ("fr", "tab_help") => "Aide",
        ("de", "tab_help") => "Hilfe",
        (_, "tab_help") => "Help",

        // Light Controls
        ("pt", "lighting_modes") => "Modos de Iluminação",
        ("es", "lighting_modes") => "Modos de Iluminación",
        ("fr", "lighting_modes") => "Modes d'Éclairage",
        ("de", "lighting_modes") => "Beleuchtungsmodi",
        (_, "lighting_modes") => "Lighting Modes",

        ("pt", "light_brightness") => "Brilho",
        ("es", "light_brightness") => "Brillo",
        ("fr", "light_brightness") => "Luminosité",
        ("de", "light_brightness") => "Helligkeit",
        (_, "light_brightness") => "Brightness",

        ("pt", "light_speed") => "Delay",
        ("es", "light_speed") => "Delay",
        ("fr", "light_speed") => "Délai",
        ("de", "light_speed") => "Tempo",
        (_, "light_speed") => "Speed",

        // Music visualizer
        ("pt", "music_title") => "Visualizador de Áudio - Modo Música",
        ("es", "music_title") => "Visualizador de Audio - Modo Música",
        ("fr", "music_title") => "Visualiseur Audio - Mode Musique",
        ("de", "music_title") => "Audio-Visualisierer - Musikmodus",
        (_, "music_title") => "Music Mode Audio Visualizer",

        ("pt", "music_pattern") => "Padrão:",
        ("es", "music_pattern") => "Patrón:",
        ("fr", "music_pattern") => "Motif :",
        ("de", "music_pattern") => "Muster:",
        (_, "music_pattern") => "Pattern:",

        ("pt", "music_freq") => "Taxa de Frequência:",
        ("es", "music_freq") => "Tasa de Frecuencia:",
        ("fr", "music_freq") => "Fréquence :",
        ("de", "music_freq") => "Frequenzrate:",
        (_, "music_freq") => "Frequency Rate:",

        ("pt", "music_start") => "Iniciar Transmissão",
        ("es", "music_start") => "Iniciar Transmisión",
        ("fr", "music_start") => "Démarrer le Flux",
        ("de", "music_start") => "Visualisierer starten",
        (_, "music_start") => "Start Visualizer Stream",

        ("pt", "music_stop") => "Parar Transmissão",
        ("es", "music_stop") => "Detener Transmisión",
        ("fr", "music_stop") => "Arrêter le Flux",
        ("de", "music_stop") => "Visualisierer stoppen",
        (_, "music_stop") => "Stop Visualizer Stream",

        // Macro view
        ("pt", "macro_list") => "Lista de Macros",
        ("es", "macro_list") => "Lista de Macros",
        ("fr", "macro_list") => "Liste des Macros",
        ("de", "macro_list") => "Makroliste",
        (_, "macro_list") => "Macro List",

        ("pt", "btn_new") => "Novo",
        ("es", "btn_new") => "Nuevo",
        ("fr", "btn_new") => "Nouveau",
        ("de", "btn_new") => "Neu",
        (_, "btn_new") => "New",

        ("pt", "btn_delete") => "Excluir",
        ("es", "btn_delete") => "Eliminar",
        ("fr", "btn_delete") => "Supprimer",
        ("de", "btn_delete") => "Löschen",
        (_, "btn_delete") => "Delete",

        ("pt", "btn_copy") => "Copiar",
        ("es", "btn_copy") => "Copiar",
        ("fr", "btn_copy") => "Copier",
        ("de", "btn_copy") => "Kopieren",
        (_, "btn_copy") => "Copy",

        ("pt", "btn_import") => "Importar",
        ("es", "btn_import") => "Importar",
        ("fr", "btn_import") => "Importer",
        ("de", "btn_import") => "Importieren",
        (_, "btn_import") => "Import",

        ("pt", "btn_export") => "Exportar",
        ("es", "btn_export") => "Exportar",
        ("fr", "btn_export") => "Exporter",
        ("de", "btn_export") => "Exportieren",
        (_, "btn_export") => "Export",

        ("pt", "btn_rename") => "Renomear",
        ("es", "btn_rename") => "Renombrar",
        ("fr", "btn_rename") => "Renommer",
        ("de", "btn_rename") => "Umbenennen",
        (_, "btn_rename") => "Rename",

        ("pt", "repeat_time") => "Repetições",
        ("es", "repeat_time") => "Repeticiones",
        ("fr", "repeat_time") => "Répétitions",
        ("de", "repeat_time") => "Wiederholungen",
        (_, "repeat_time") => "Repeat Time",

        ("pt", "delay_mode") => "Modo de Intervalo:",
        ("es", "delay_mode") => "Modo de Retardo:",
        ("fr", "delay_mode") => "Mode de Délai :",
        ("de", "delay_mode") => "Verzögerungsmodus:",
        (_, "delay_mode") => "Delay Mode:",

        ("pt", "delay_record") => "Gravar Intervalo",
        ("es", "delay_record") => "Grabar Retardo",
        ("fr", "delay_record") => "Enregistrer le Délai",
        ("de", "delay_record") => "Verzögerung aufnehmen",
        (_, "delay_record") => "Record Delay",

        ("pt", "delay_none") => "Sem Intervalo",
        ("es", "delay_none") => "Sin Retardo",
        ("fr", "delay_none") => "Sans Délai",
        ("de", "delay_none") => "Keine Verzögerung",
        (_, "delay_none") => "No Delay",

        ("pt", "delay_default") => "Padrão",
        ("es", "delay_default") => "Predeterminado",
        ("fr", "delay_default") => "Par Défaut",
        ("de", "delay_default") => "Standard",
        (_, "delay_default") => "Default",

        ("pt", "delay_ms") => "MS",
        ("es", "delay_ms") => "MS",
        ("fr", "delay_ms") => "MS",
        ("de", "delay_ms") => "MS",
        (_, "delay_ms") => "MS",

        ("pt", "macro_record") => "Gravação de Macro",
        ("es", "macro_record") => "Grabación de Macro",
        ("fr", "macro_record") => "Enregistrement de Macro",
        ("de", "macro_record") => "Makroaufnahme",
        (_, "macro_record") => "Macro Record",

        ("pt", "col_desc") => "Descrição",
        ("es", "col_desc") => "Descripción",
        ("fr", "col_desc") => "Description",
        ("de", "col_desc") => "Beschreibung",
        (_, "col_desc") => "Description",

        ("pt", "col_action") => "Ação",
        ("es", "col_action") => "Acción",
        ("fr", "col_action") => "Action",
        ("de", "col_action") => "Aktion",
        (_, "col_action") => "Action",

        ("pt", "col_delay") => "Intervalo(ms)",
        ("es", "col_delay") => "Retardo(ms)",
        ("fr", "col_delay") => "Délai(ms)",
        ("de", "col_delay") => "Verzögerung(ms)",
        (_, "col_delay") => "Delay(ms)",

        ("pt", "btn_record") => "Gravar",
        ("es", "btn_record") => "Grabar",
        ("fr", "btn_record") => "Enregistrer",
        ("de", "btn_record") => "Aufnehmen",
        (_, "btn_record") => "Record",

        ("pt", "btn_stop") => "Parar",
        ("es", "btn_stop") => "Detener",
        ("fr", "btn_stop") => "Arrêter",
        ("de", "btn_stop") => "Stopp",
        (_, "btn_stop") => "Stop",

        ("pt", "btn_play") => "Executar",
        ("es", "btn_play") => "Reproducir",
        ("fr", "btn_play") => "Lire",
        ("de", "btn_play") => "Abspielen",
        (_, "btn_play") => "Play",

        ("pt", "btn_save") => "Salvar",
        ("es", "btn_save") => "Guardar",
        ("fr", "btn_save") => "Enregistrer",
        ("de", "btn_save") => "Speichern",
        (_, "btn_save") => "Save",

        ("pt", "action_down") => "Down",
        ("es", "action_down") => "Down",
        ("fr", "action_down") => "Down",
        ("de", "action_down") => "Down",
        (_, "action_down") => "Down",

        ("pt", "action_up") => "Up",
        ("es", "action_up") => "Up",
        ("fr", "action_up") => "Up",
        ("de", "action_up") => "Up",
        (_, "action_up") => "Up",

        ("pt", "hint_recording") => "● Gravando teclas... Pressione 'Parar' ao terminar.",
        ("es", "hint_recording") => "● Grabando teclas... Presione 'Detener' al finalizar.",
        ("fr", "hint_recording") => "● Enregistrement des touches... Cliquez sur 'Arrêter'.",
        ("de", "hint_recording") => "● Tastendrücke aufnehmen... Klicken Sie auf 'Stopp'.",
        (_, "hint_recording") => "● Recording keystrokes... Press 'Stop' when done.",

        // System Config
        ("pt", "language") => "Idioma",
        ("es", "language") => "Idioma",
        ("fr", "language") => "Langue",
        ("de", "language") => "Sprache",
        (_, "language") => "Language",

        ("pt", "auto_run") => "Iniciar com o Sistema",
        ("es", "auto_run") => "Iniciar con el Sistema",
        ("fr", "auto_run") => "Démarrage Automatique",
        ("de", "auto_run") => "Autostart mit System",
        (_, "auto_run") => "Auto Run",

        ("pt", "restore_factory") => "Redefinir Configurações",
        ("es", "restore_factory") => "Restablecer Configuración",
        ("fr", "restore_factory") => "Réinitialiser les Paramètres",
        ("de", "restore_factory") => "Einstellungen zurücksetzen",
        (_, "restore_factory") => "Reset Settings",

        ("pt", "msg_factory_reset_title") => "Restauração de Fábrica",
        ("es", "msg_factory_reset_title") => "Restablecimiento de Fábrica",
        ("fr", "msg_factory_reset_title") => "Réinitialisation d'Usine",
        ("de", "msg_factory_reset_title") => "Auf Werkseinstellungen zurücksetzen",
        (_, "msg_factory_reset_title") => "Factory Reset",

        ("pt", "msg_factory_reset_confirm") => "Restaurar as configurações padrão de fábrica?\n\nEsta ação não pode ser desfeita.",
        ("es", "msg_factory_reset_confirm") => "¿Restablecer la configuración predeterminada de fábrica?\n\nEsta acción no se puede deshacer.",
        ("fr", "msg_factory_reset_confirm") => "Restaurer les paramètres d'usine par défaut ?\n\nCette action est irréversible.",
        ("de", "msg_factory_reset_confirm") => "Werkseinstellungen wiederherstellen?\n\nDies kann nicht rückgängig gemacht werden.",
        (_, "msg_factory_reset_confirm") => "Restore factory default settings?\n\nThis cannot be undone.",

        // Connection States
        ("pt", "dev_connected") => "Dispositivo conectado",
        ("es", "dev_connected") => "Dispositivo conectado",
        ("fr", "dev_connected") => "Périphérique connecté",
        ("de", "dev_connected") => "Gerät verbunden",
        (_, "dev_connected") => "Device connected",

        ("pt", "dev_disconnected") => "Dispositivo desconectado",
        ("es", "dev_disconnected") => "Dispositivo desconectado",
        ("fr", "dev_disconnected") => "Périphérique déconnecté",
        ("de", "dev_disconnected") => "Gerät getrennt",
        (_, "dev_disconnected") => "Device disconnected",

        ("pt", "dev_perm_required") => "Permissões necessárias",
        ("es", "dev_perm_required") => "Permisos requeridos",
        ("fr", "dev_perm_required") => "Permissions requises",
        ("de", "dev_perm_required") => "Berechtigungen erforderlich",
        (_, "dev_perm_required") => "Permissions required",

        ("pt", "dev_captured_vm") => "Dispositivo capturado pela VM",
        ("es", "dev_captured_vm") => "Dispositivo capturado por VM",
        ("fr", "dev_captured_vm") => "Périphérique capturé par la VM",
        ("de", "dev_captured_vm") => "Gerät von VM erfasst",
        (_, "dev_captured_vm") => "Device captured by VM",

        ("pt", "dev_badge_connected") => "FREE WOLF K8 USB",
        ("es", "dev_badge_connected") => "FREE WOLF K8 USB",
        ("fr", "dev_badge_connected") => "FREE WOLF K8 USB",
        ("de", "dev_badge_connected") => "FREE WOLF K8 USB",
        (_, "dev_badge_connected") => "FREE WOLF K8 USB",

        ("pt", "dev_badge_disconnected") => "NENHUM DISPOSITIVO DETECTADO",
        ("es", "dev_badge_disconnected") => "NINGÚN DISPOSITIVO DETECTADO",
        ("fr", "dev_badge_disconnected") => "AUCUN PÉRIPHÉRIQUE DÉTECTÉ",
        ("de", "dev_badge_disconnected") => "KEIN GERÄT ERKANNT",
        (_, "dev_badge_disconnected") => "NO DEVICE DETECTED",

        ("pt", "btn_fix_udev") => "Corrigir Permissões",
        ("es", "btn_fix_udev") => "Corregir Permisos",
        ("fr", "btn_fix_udev") => "Régler Permissions",
        ("de", "btn_fix_udev") => "Rechte beheben",
        (_, "btn_fix_udev") => "Fix Permissions",

        ("pt", "help_title") => "Manual e Ajuda",
        ("es", "help_title") => "Manual y Ayuda",
        ("fr", "help_title") => "Manuel et Aide",
        ("de", "help_title") => "Handbuch und Hilfe",
        (_, "help_title") => "Manual & Help",

        ("pt", "btn_prev") => "◀ Anterior",
        ("es", "btn_prev") => "◀ Anterior",
        ("fr", "btn_prev") => "◀ Précédent",
        ("de", "btn_prev") => "◀ Zurück",
        (_, "btn_prev") => "◀ Prev",

        ("pt", "btn_next") => "Próximo ▶",
        ("es", "btn_next") => "Siguiente ▶",
        ("fr", "btn_next") => "Suivant ▶",
        ("de", "btn_next") => "Weiter ▶",
        (_, "btn_next") => "Next ▶",

        _ => key,
    }
}

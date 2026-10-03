//! User Manual and Help Documentation for FREE WOLF K8 Keyboard

#[derive(Debug, Clone)]
pub struct TopicInfo {
    pub id: usize,
    pub icon: &'static str,
    pub title: &'static str,
    pub content: &'static str,
}

pub fn get_topics(lang: &str) -> Vec<TopicInfo> {
    match lang {
        "pt" => vec![
            TopicInfo {
                id: 0,
                icon: "📋",
                title: "Visão Geral e Specs",
                content: "=== VISÃO GERAL DO TECLADO FREEWOLF K8 ===\n\n\
                O teclado mecânico FREEWOLF K8 é um teclado gamer tri-modo de alto desempenho.\n\n\
                ESPECIFICAÇÕES TÉCNICAS:\n\
                • Modelo: FREEWOLF K8\n\
                • Teclas: 100 teclas (layout compacto com teclado numérico)\n\
                • Switches: Hot-Swap de 3 e 5 pinos\n\
                • Conexões: USB Tipo-C, 2.4 GHz sem fio, Bluetooth 5.0 (3 perfis)\n\
                • Bateria: Lítio 4000 mAh recarregável\n\
                • Iluminação: RGB com 20 modos dinâmicos e reativo ao som\n\
                • Compatibilidade: Linux, Windows, macOS, Android, iOS\n\
                • Taxa de Polling: 1000 Hz (Cabo / 2.4G), 125 Hz (Bluetooth)",
            },
            TopicInfo {
                id: 1,
                icon: "📶",
                title: "Conectividade Tri-Modo",
                content: "=== CONECTIVIDADE TRI-MODO ===\n\n\
                O teclado suporta três modos de conexão selecionáveis pela chave física traseira:\n\n\
                1. MODO CABO USB:\n\
                   • Coloque a chave traseira na posição central (OFF/Cabo).\n\
                   • Conecte o cabo USB Tipo-C ao computador.\n\n\
                2. MODO SEM FIO 2.4 GHz:\n\
                   • Coloque a chave traseira na posição 'G' (2.4G).\n\
                   • Conecte o receptor USB ao computador.\n\
                   • Pressione Fn + R por 3 segundos para emparelhar se necessário.\n\n\
                3. MODO BLUETOOTH 5.0:\n\
                   • Coloque a chave traseira na posição 'B' (Bluetooth).\n\
                   • Fn + Q: Perfil Bluetooth 1 (LED azul)\n\
                   • Fn + W: Perfil Bluetooth 2 (LED ciano)\n\
                   • Fn + E: Perfil Bluetooth 3 (LED roxo)\n\
                   • Segure Fn + Q/W/E por 3 a 5 segundos para entrar em modo de pareamento.",
            },
            TopicInfo {
                id: 2,
                icon: "💻",
                title: "Modos Windows e Mac",
                content: "=== SISTEMAS OPERACIONAIS E ATALHOS ===\n\n\
                Alternância de Layout:\n\
                • Fn + A: Modo Windows (layout PC padrão, tecla Win ativa).\n\
                • Fn + S: Modo Mac (inverte as teclas Option e Command para layout nativo Apple).\n\
                • Fn + Win: Bloqueio da tecla Windows (Modo Gamer).\n\n\
                Teclas Multimídia F1 - F12:\n\
                • Fn + F1: Reprodutor de mídia\n\
                • Fn + F2: Diminuir volume\n\
                • Fn + F3: Aumentar volume\n\
                • Fn + F4: Silenciar (Mute)\n\
                • Fn + F5: Faixa anterior\n\
                • Fn + F6: Próxima faixa\n\
                • Fn + F7: Reproduzir / Pausar\n\
                • Fn + F8: Parar reprodução\n\
                • Fn + F9: Navegador Web\n\
                • Fn + F10: Email\n\
                • Fn + F11: Meu Computador / Arquivos\n\
                • Fn + F12: Calculadora",
            },
            TopicInfo {
                id: 3,
                icon: "✨",
                title: "Iluminação RGB e Efeitos",
                content: "=== CONTROLE DE ILUMINAÇÃO RGB ===\n\n\
                Atalhos de Iluminação:\n\
                • Fn + \\| : Alternar entre os 19 efeitos de iluminação integrados.\n\
                • Fn + ↑ / ↓ : Ajustar o brilho (5 níveis, incluindo desligado).\n\
                • Fn + ← / → : Ajustar a velocidade da animação (5 níveis).\n\
                • Fn + Backspace : Ligar / Desligar toda a iluminação.\n\
                • Fn + ~ : Gravação de mapa de iluminação customizado.\n\n\
                No aplicativo Linux, você pode selecionar diretamente qualquer um dos 20 modos,\n\
                incluindo o modo Música com visualizador em tempo real.",
            },
            TopicInfo {
                id: 4,
                icon: "🎯",
                title: "Perfis Gamer e DIY",
                content: "=== PERFIS GAMER E ILUMINAÇÃO CUSTOMIZADA ===\n\n\
                Presets de Iluminação para Jogos:\n\
                • Fn + 1: Modo FPS (W, A, S, D e setas direcionais iluminados).\n\
                • Fn + 2: Modo LOL / MOBA (Q, W, E, R, D, F, B, 1-7 iluminados).\n\
                • Fn + 3: Modo Escritório (37 teclas principais de digitação iluminadas).\n\n\
                Gravando Iluminação Customizada (DIY):\n\
                1. Pressione Fn + 1, 2 ou 3 para escolher o slot de gravação.\n\
                2. Segure Fn + ~ até que o indicador pisque para entrar no modo gravação.\n\
                3. Pressione repetidamente cada tecla desejada para mudar sua cor.\n\
                4. Pressione Fn + ~ novamente para salvar na memória EEPROM do teclado.",
            },
            TopicInfo {
                id: 5,
                icon: "🔋",
                title: "Bateria e Economia",
                content: "=== BATERIA E GERENCIAMENTO DE ENERGIA ===\n\n\
                • Bateria interna: 4000 mAh de polímero de lítio.\n\
                • Autonomia:\n\
                  - Com iluminação RGB ligada: até 30 horas contínuas.\n\
                  - Com iluminação desligada: até 200 horas.\n\
                • Suspensão Inteligente:\n\
                  - O teclado entra em modo de economia após 2 minutos sem uso.\n\
                  - Suspensão profunda após 30 minutos.\n\
                  - Qualquer tecla reativa instantaneamente sem perda de digitação.\n\
                • Indicador de Carga: o LED sob a barra de espaço ou tecla FN pisca em vermelho\n\
                  durante o carregamento e fica verde quando totalmente carregado.",
            },
            TopicInfo {
                id: 6,
                icon: "🔧",
                title: "Troca de Switches",
                content: "=== HOT-SWAP E MANUTENÇÃO DOS SWITCHES ===\n\n\
                O FREEWOLF K8 possui soquetes Hot-Swap padrão compatíveis com a maioria dos\n\
                switches mecânicos de 3 pinos e 5 pinos (estilo MX, Cherry, Gateron, Outemu, Kailh).\n\n\
                Passo a passo para troca:\n\
                1. Desconecte o teclado do computador ou desligue a chave traseira.\n\
                2. Use o extrator de keycaps para remover cuidadosamente a capa da tecla.\n\
                3. Encaixe o extrator metálico de switches nas travas superior e inferior do switch.\n\
                4. Puxe para cima suavemente sem torcer.\n\
                5. Certifique-se de que os pinos de metal do novo switch estão retos antes de inserir.\n\
                6. Pressione o novo switch no soquete até ouvir um clique firme.",
            },
            TopicInfo {
                id: 7,
                icon: "❓",
                title: "Solução de Problemas",
                content: "=== SOLUÇÃO DE PROBLEMAS FREQUENTES ===\n\n\
                1. O teclado não responde no Linux:\n\
                   • Verifique a posição da chave traseira (central para USB com fio).\n\
                   • Execute 'k8ctl setup-udev' para conceder permissões de acesso ao /dev/hidraw.\n\n\
                2. Teclado capturado por máquina virtual (QEMU/KVM):\n\
                   • Feche o QEMU ou desanexe o dispositivo USB 1a2c:7c80 da VM convidada.\n\n\
                3. Falha na reprodução de macros:\n\
                   • O Linux requer permissões de gravação em /dev/uinput para injetar teclas.\n\
                   • Execute 'k8ctl setup-udev' para corrigir automaticamente.\n\n\
                4. Reset de Fábrica via Teclado:\n\
                   • Segure Fn + Barra de Espaço por 3 segundos para resetar o teclado.",
            },
            TopicInfo {
                id: 8,
                icon: "🐧",
                title: "Driver Linux e CLI",
                content: "=== COMANDOS E RECURSOS DO DRIVER LINUX ===\n\n\
                O aplicativo nativo FREE WOLF K8 oferece controle completo via interface gráfica (GUI)\n\
                e via terminal (CLI) em um único binário executável.\n\n\
                Comandos rápidos no terminal:\n\
                • k8ctl status              : Exibe o status da conexão USB e permissões.\n\
                • k8ctl setup-udev          : Instala as regras de permissão udev automaticamente.\n\
                • k8ctl list                : Lista todos os 21 modos de iluminação.\n\
                • k8ctl set steady -b 4     : Define modo Estático no brilho máximo.\n\
                • k8ctl set breathing -s 2  : Define modo Respiração na velocidade 2.\n\
                • k8ctl set 0               : Desliga toda a iluminação (economia de bateria).\n\
                • k8ctl music -m 2          : Inicia o visualizador de áudio em tempo real.\n\
                • k8ctl macro list          : Lista macros gravados.",
            },
        ],
        _ => vec![
            TopicInfo {
                id: 0,
                icon: "📋",
                title: "Overview & Specs",
                content: "=== FREE WOLF K8 KEYBOARD OVERVIEW ===\n\n\
                The FREE WOLF K8 is a high-performance tri-mode mechanical gaming keyboard.\n\n\
                HARDWARE SPECIFICATIONS:\n\
                • Model: FREE WOLF K8\n\
                • Keys: 100 Keys (compact 96% layout with dedicated numeric keypad)\n\
                • Switches: Hot-Swappable 3-pin & 5-pin MX compatible sockets\n\
                • Connectivity: USB Type-C, 2.4 GHz Wireless, Bluetooth 5.0 (3 channels)\n\
                • Battery: 4000 mAh rechargeable lithium polymer battery\n\
                • Lighting: Dynamic RGB with 20 preset hardware effects + Audio Visualizer\n\
                • System Compatibility: Linux, Windows, macOS, Android, iOS\n\
                • Polling Rate: 1000 Hz (Wired / 2.4G), 125 Hz (Bluetooth)",
            },
            TopicInfo {
                id: 1,
                icon: "📶",
                title: "Tri-Mode Connectivity",
                content: "=== TRI-MODE CONNECTIVITY GUIDE ===\n\n\
                Select the hardware connection mode using the physical switch on the rear panel:\n\n\
                1. WIRED USB MODE:\n\
                   • Set the rear switch to the center position (OFF/Wired).\n\
                   • Connect the USB Type-C cable to your PC.\n\n\
                2. 2.4 GHz WIRELESS MODE:\n\
                   • Set the rear switch to 'G' (2.4G).\n\
                   • Insert the USB wireless receiver into your PC.\n\
                   • Press and hold Fn + R for 3 seconds to re-pair if needed.\n\n\
                3. BLUETOOTH 5.0 MODE:\n\
                   • Set the rear switch to 'B' (Bluetooth).\n\
                   • Fn + Q: Bluetooth Channel 1 (Blue LED)\n\
                   • Fn + W: Bluetooth Channel 2 (Cyan LED)\n\
                   • Fn + E: Bluetooth Channel 3 (Purple LED)\n\
                   • Press and hold Fn + Q/W/E for 3–5 seconds to enter pairing mode.",
            },
            TopicInfo {
                id: 2,
                icon: "💻",
                title: "Windows & Mac Layout",
                content: "=== OPERATING SYSTEM LAYOUTS & HOTKEYS ===\n\n\
                OS Profile Switching:\n\
                • Fn + A: Switch to Windows Mode (Standard PC layout, Windows key active).\n\
                • Fn + S: Switch to macOS Mode (Swaps Option and Command keys for native Mac layout).\n\
                • Fn + Win: Windows Key Lock / Unlock (Gaming Mode: prevents accidental desktop minimization).\n\n\
                F1 – F12 Multimedia Hotkeys:\n\
                • Fn + F1: Default Media Player\n\
                • Fn + F2: Volume Down\n\
                • Fn + F3: Volume Up\n\
                • Fn + F4: Mute Audio\n\
                • Fn + F5: Previous Track\n\
                • Fn + F6: Next Track\n\
                • Fn + F7: Play / Pause\n\
                • Fn + F8: Stop Playback\n\
                • Fn + F9: Web Browser\n\
                • Fn + F10: Email Client\n\
                • Fn + F11: File Manager / This PC\n\
                • Fn + F12: Calculator",
            },
            TopicInfo {
                id: 3,
                icon: "✨",
                title: "RGB Lighting Controls",
                content: "=== RGB LIGHTING SHORTCUTS & MODES ===\n\n\
                On-Keyboard Lighting Controls:\n\
                • Fn + \\| : Cycle through 19 dynamic built-in hardware lighting effects.\n\
                • Fn + ↑ / ↓ : Adjust brightness levels (5 steps: 0% / Off to 100%).\n\
                • Fn + ← / → : Adjust animation speed (5 dynamic steps).\n\
                • Fn + Backspace : Toggle all backlights On / Off (Power Saving).\n\
                • Fn + ~ : Record custom key lighting map to onboard EEPROM.\n\n\
                In the Linux application, you can directly activate any of the 20 lighting modes,\n\
                including the real-time Music Mode spectrum visualizer.",
            },
            TopicInfo {
                id: 4,
                icon: "🎯",
                title: "Gaming & Custom Keys",
                content: "=== GAMING PRESETS & ONBOARD RECORDING ===\n\n\
                Gaming Backlighting Presets:\n\
                • Fn + 1: FPS Mode (W, A, S, D, and Arrow keys illuminated).\n\
                • Fn + 2: LOL / MOBA Mode (Q, W, E, R, D, F, B, 1-7 illuminated).\n\
                • Fn + 3: Office Mode (37 primary typing keys illuminated).\n\n\
                Custom Lighting Map Recording (DIY):\n\
                1. Press Fn + 1, 2, or 3 to select the preset slot.\n\
                2. Hold Fn + ~ until the indicator LED flashes to enter recording mode.\n\
                3. Press each individual key repeatedly to cycle through colors.\n\
                4. Press Fn + ~ again to save the custom map to onboard keyboard memory.",
            },
            TopicInfo {
                id: 5,
                icon: "🔋",
                title: "Battery & Power Saving",
                content: "=== BATTERY & POWER MANAGEMENT ===\n\n\
                • Battery Capacity: 4000 mAh rechargeable lithium polymer.\n\
                • Battery Life:\n\
                  - RGB active: up to 30 continuous hours.\n\
                  - RGB off: up to 200 continuous hours.\n\
                • Intelligent Sleep Timer:\n\
                  - Light sleep after 2 minutes of idle time.\n\
                  - Deep sleep after 30 minutes of idle time.\n\
                  - Any keystroke wakes the keyboard instantly without input lag or missed keys.\n\
                • Charging Indicator: Red LED blinks under spacebar during charge, turns green when full.",
            },
            TopicInfo {
                id: 6,
                icon: "🔧",
                title: "Hot-Swap & Switches",
                content: "=== HOT-SWAPPABLE SWITCHES & MAINTENANCE ===\n\n\
                The FREE WOLF K8 features universal Hot-Swap sockets compatible with almost all\n\
                3-pin and 5-pin mechanical switches (Cherry MX, Gateron, Outemu, Kailh, etc.).\n\n\
                How to swap a switch:\n\
                1. Disconnect the USB cable or power off the keyboard.\n\
                2. Use the wire keycap puller to remove the keycap vertically.\n\
                3. Align the metal switch puller with the top and bottom retaining clips of the switch.\n\
                4. Squeeze and pull straight up.\n\
                5. Verify that the two metal pins on the new switch are completely straight.\n\
                6. Align pins with the PCB socket holes and push down firmly until it clicks.",
            },
            TopicInfo {
                id: 7,
                icon: "❓",
                title: "Troubleshooting Guide",
                content: "=== LINUX TROUBLESHOOTING GUIDE ===\n\n\
                1. Device Not Detected:\n\
                   • Ensure the rear switch is in the center position for wired USB mode.\n\
                   • Run 'k8ctl setup-udev' to configure permissions for /dev/hidraw.\n\n\
                2. Device Claimed by Virtual Machine (QEMU/KVM):\n\
                   • Close QEMU or detach USB device 1a2c:7c80 from the guest OS.\n\n\
                3. Virtual Keyboard Macro Playback Fails:\n\
                   • Macro playback requires write permissions to /dev/uinput.\n\
                   • Run 'k8ctl setup-udev' to automatically install permissions.\n\n\
                4. Factory Reset via Hardware Shortcut:\n\
                   • Press and hold Fn + Spacebar for 3 seconds to restore default factory settings.",
            },
            TopicInfo {
                id: 8,
                icon: "🐧",
                title: "Linux Driver & CLI",
                content: "=== LINUX DRIVER & CLI FEATURES ===\n\n\
                The native FREE WOLF K8 application provides complete control via both Graphical (GUI)\n\
                and Command-Line (CLI) modes within a single high-performance binary.\n\n\
                Quick CLI Commands:\n\
                • k8ctl status              : Display connection, permissions, and device node.\n\
                • k8ctl setup-udev          : Install udev rules and configure /dev/uinput access.\n\
                • k8ctl list                : List all 21 lighting modes with parameters.\n\
                • k8ctl set steady -b 4     : Set full static backlighting.\n\
                • k8ctl set breathing -s 2  : Set Breathing mode at speed 2.\n\
                • k8ctl set 0               : Turn off all RGB backlights (power saver).\n\
                • k8ctl music -m 2          : Launch real-time live audio spectrum visualizer.\n\
                • k8ctl macro list          : View all saved macros.",
            },
        ],
    }
}

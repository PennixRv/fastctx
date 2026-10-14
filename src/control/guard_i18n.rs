//! Localized copy for provider-derived output protection.

use super::i18n::Language;
use super::provider::GuardReason;

/// Complete provider-guard copy for one supported language.
#[derive(Debug)]
pub(crate) struct GuardMessages {
    pub(crate) section_title: &'static str,
    pub(crate) label: &'static str,
    /// Active copy for known local runtimes such as Ollama and LM Studio.
    pub(crate) active_note: &'static str,
    /// Active copy for custom relay routes, including providers named OpenAI with a base URL.
    pub(crate) relay_active_note: &'static str,
    pub(crate) disabled_note: &'static str,
    pub(crate) available_note: &'static str,
    pub(crate) disable_confirm: &'static str,
}

impl GuardMessages {
    #[cfg(test)]
    fn values(&self) -> [&'static str; 7] {
        [
            self.section_title,
            self.label,
            self.active_note,
            self.relay_active_note,
            self.disabled_note,
            self.available_note,
            self.disable_confirm,
        ]
    }

    pub(crate) const fn active_note(&self, reason: Option<GuardReason>) -> &'static str {
        match reason {
            Some(GuardReason::UnverifiedRelay) => self.relay_active_note,
            Some(GuardReason::LocalCompaction) | None => self.active_note,
        }
    }
}

/// Returns provider-guard copy for a supported language.
pub(crate) const fn messages(language: Language) -> &'static GuardMessages {
    match language {
        Language::En => &EN,
        Language::ZhCn => &ZH_CN,
        Language::ZhTw => &ZH_TW,
        Language::Ja => &JA,
        Language::Ko => &KO,
        Language::Es => &ES,
        Language::Fr => &FR,
        Language::De => &DE,
        Language::PtBr => &PT_BR,
        Language::Ru => &RU,
        Language::It => &IT,
        Language::Tr => &TR,
        Language::Pl => &PL,
        Language::Nl => &NL,
        Language::Vi => &VI,
        Language::Id => &ID,
        Language::Uk => &UK,
    }
}

macro_rules! guard_messages {
    ($section:expr, $label:expr, $active:expr, $relay_active:expr, $disabled:expr, $available:expr,
     $confirm:expr) => {
        GuardMessages {
            section_title: $section,
            label: $label,
            active_note: $active,
            relay_active_note: $relay_active,
            disabled_note: $disabled,
            available_note: $available,
            disable_confirm: $confirm,
        }
    };
}

const EN: GuardMessages = guard_messages!(
    "Provider compatibility",
    "Provider guard",
    "This provider uses local compaction: Codex refeeds the full history to the model. Guarded keeps the selected tier and individual tool budgets; it does not impose a shared per-turn output pool and cannot verify the model catalog or context threshold.",
    "This provider is a relay whose compaction protocol and model catalog FastCtx cannot verify. Guarded keeps the selected tier and individual tool budgets; it does not impose a shared per-turn output pool or replace correct relay usage, compaction, and context-error handling.",
    "Protection is disabled. FastCtx still follows the selected tier, but it cannot verify the provider's catalog, usage, compaction, or context-error contract.",
    "Protection is on but idle: this route is official OpenAI, Azure, or Bedrock. The selected tier remains effective; Guarded only identifies local or unverified relay routes.",
    "Disable provider output protection?\nFastCtx will still follow the selected tier, but it cannot verify provider usage, compaction, catalog, or context-error handling. This affects Apply and new runtime sessions."
);
const ZH_CN: GuardMessages = guard_messages!(
    "Provider 兼容性",
    "Provider 输出保护",
    "当前 provider 使用本地压缩：Codex 会把整段历史重新交给模型。Guarded 保持你选择的档位和各工具单次预算，不创建每轮共享输出池；它无法验证模型目录或上下文阈值。",
    "当前 provider 是 FastCtx 无法验证压缩协议与模型目录的中转。Guarded 保持你选择的档位和各工具单次预算，不创建每轮共享输出池，也不能替代中转对 usage、压缩和上下文错误码的正确实现。",
    "保护已关闭。FastCtx 仍遵循你选择的档位，但无法验证 provider 的目录、usage、压缩和上下文错误契约。",
    "保护已开启，当前路线无需额外收紧：这是官方 OpenAI、Azure 或 Bedrock 路线；本地或未经验证的中转路线仍会显示 Guarded 兼容性提示。",
    "要关闭 provider 输出保护吗？\nFastCtx 仍会遵循所选档位，但无法验证 provider 的 usage、压缩、目录或上下文错误处理。这会影响 Apply 和新启动的运行时会话。"
);
const ZH_TW: GuardMessages = guard_messages!(
    "Provider 相容性",
    "Provider 輸出保護",
    "目前的 provider 使用本機壓縮：Codex 會把整段歷史重新交給模型。Guarded 保持所選層級和各工具單次預算，不建立每回合共用輸出池；它無法驗證模型目錄或上下文門檻。",
    "目前的 provider 是 FastCtx 無法驗證壓縮協定與模型目錄的中轉。Guarded 保持所選層級和各工具單次預算，不建立每回合共用輸出池，也不能取代中轉對 usage、壓縮與上下文錯誤碼的正確實作。",
    "保護已停用。FastCtx 仍遵循所選層級，但無法驗證 provider 的目錄、usage、壓縮和上下文錯誤契約。",
    "保護已啟用，目前路線無需額外收緊：這是官方 OpenAI、Azure 或 Bedrock 路線；本機或未經驗證的中轉路線仍會顯示 Guarded 相容性提示。",
    "要停用 provider 輸出保護嗎？\nFastCtx 仍會遵循所選層級，但無法驗證 provider 的 usage、壓縮、目錄或上下文錯誤處理。這會影響 Apply 與新的執行階段工作階段。"
);
const JA: GuardMessages = guard_messages!(
    "Provider 互換性",
    "Provider 出力保護",
    "この provider はローカル圧縮を使い、Codex は履歴全体をモデルへ再送します。Guarded は選択した段階と各ツールの単回予算を維持し、ターン共有プールを設けません。モデルカタログやコンテキスト閾値は検証できません。",
    "この provider は、FastCtx が圧縮プロトコルとモデルカタログを検証できない中継です。Guarded は選択した段階と各ツールの単回予算を維持し、ターン共有プールを設けず、中継側の usage・圧縮・コンテキストエラー処理を代替しません。",
    "保護は無効です。FastCtx は選択した段階に従いますが、provider のカタログ、usage、圧縮、コンテキストエラー契約は検証できません。",
    "保護は有効ですが現在は待機中です。公式 OpenAI、Azure、Bedrock の経路では選択した段階が有効で、ローカルまたは未検証の中継経路では Guarded の互換性表示が有効になります。",
    "provider 出力保護を無効にしますか？\nFastCtx は選択した段階に従いますが、provider の usage・圧縮・カタログ・コンテキストエラー処理は検証できません。Apply と新しいランタイムセッションに影響します。"
);
const KO: GuardMessages = guard_messages!(
    "Provider 호환성",
    "Provider 출력 보호",
    "이 provider는 로컬 압축을 사용하므로 Codex가 전체 기록을 모델에 다시 보냅니다. Guarded는 선택한 단계와 각 도구의 단일 예산을 유지하며 턴 공유 풀을 만들지 않습니다. 모델 카탈로그나 컨텍스트 임계값은 검증하지 못합니다.",
    "이 provider는 FastCtx가 압축 프로토콜과 모델 카탈로그를 검증할 수 없는 중계입니다. Guarded는 선택한 단계와 각 도구의 단일 예산을 유지하며 턴 공유 풀을 만들지 않고 중계의 usage, 압축, 컨텍스트 오류 처리를 대신하지 않습니다.",
    "보호가 꺼져 있습니다. FastCtx는 선택한 단계에 따르지만 provider의 카탈로그, usage, 압축, 컨텍스트 오류 계약은 검증하지 못합니다.",
    "보호가 켜져 있지만 현재는 대기 중입니다. 공식 OpenAI, Azure, Bedrock 경로에서는 선택한 단계가 유지되며 로컬 또는 검증되지 않은 중계 경로에서는 Guarded 호환성 표시가 활성화됩니다.",
    "provider 출력 보호를 끌까요?\nFastCtx는 선택한 단계에 따르지만 provider의 usage, 압축, 카탈로그, 컨텍스트 오류 처리는 검증하지 못합니다. Apply와 새 런타임 세션에 영향을 줍니다."
);
const ES: GuardMessages = guard_messages!(
    "Compatibilidad del provider",
    "Protección del provider",
    "Este provider usa compactación local: Codex reenvía todo el historial al modelo. Guarded conserva el nivel elegido y los presupuestos individuales; no aplica un fondo compartido por turno y no puede verificar el catálogo ni el umbral de contexto.",
    "Este provider es un relay cuyo protocolo de compactación y catálogo de modelos FastCtx no puede verificar. Guarded conserva el nivel elegido y los presupuestos individuales; no aplica un fondo compartido por turno ni sustituye el manejo correcto del relay.",
    "La protección está desactivada. FastCtx sigue el nivel elegido, pero no puede verificar el catálogo, usage, compactación ni errores de contexto del provider.",
    "La protección está activa pero inactiva: esta ruta es OpenAI oficial, Azure o Bedrock. El nivel elegido sigue vigente; Guarded solo identifica rutas locales o relays sin verificar.",
    "¿Desactivar la protección de salida del provider?\nFastCtx seguirá el nivel elegido, pero no puede verificar usage, compactación, catálogo ni errores de contexto del provider. Afecta a Apply y a las nuevas sesiones."
);
const FR: GuardMessages = guard_messages!(
    "Compatibilité du provider",
    "Protection du provider",
    "Ce provider utilise le compactage local : Codex renvoie tout l’historique au modèle. Guarded conserve le niveau choisi et les budgets individuels ; il n’applique pas de pool partagé par tour et ne peut pas vérifier le catalogue ni le seuil de contexte.",
    "Ce provider est un relais dont FastCtx ne peut vérifier ni le protocole de compactage ni le catalogue des modèles. Guarded conserve le niveau choisi et les budgets individuels ; il n’applique pas de pool partagé par tour et ne remplace pas la gestion correcte du relais.",
    "La protection est désactivée. FastCtx suit le niveau choisi, mais ne peut pas vérifier l’usage, le compactage, le catalogue ni les erreurs de contexte du provider.",
    "La protection est active mais en veille : cette route est OpenAI officielle, Azure ou Bedrock. Le niveau choisi reste actif ; Guarded signale seulement les routes locales ou les relais non vérifiés.",
    "Désactiver la protection de sortie du provider ?\nFastCtx suivra le niveau choisi, mais ne peut pas vérifier l’usage, le compactage, le catalogue ni les erreurs de contexte du provider. Cela touche Apply et les nouvelles sessions."
);
const DE: GuardMessages = guard_messages!(
    "Provider-Kompatibilität",
    "Provider-Schutz",
    "Dieser Provider verwendet lokale Komprimierung: Codex reicht den gesamten Verlauf erneut an das Modell. Guarded behält die gewählte Stufe und die einzelnen Tool-Budgets bei; es gibt keinen gemeinsamen Pool pro Runde und der Modellkatalog bleibt unverifiziert.",
    "Dieser Provider ist ein Relay, dessen Komprimierungsprotokoll und Modellkatalog FastCtx nicht prüfen kann. Guarded behält die gewählte Stufe und die einzelnen Tool-Budgets bei; es gibt keinen gemeinsamen Pool pro Runde und ersetzt keine korrekte Relay-Behandlung.",
    "Der Schutz ist deaktiviert. FastCtx folgt weiterhin der gewählten Stufe, kann aber Katalog, Usage, Komprimierung und Kontextfehler des Providers nicht prüfen.",
    "Der Schutz ist aktiv, aber inaktiv: Diese Route ist offizielles OpenAI, Azure oder Bedrock. Die gewählte Stufe gilt weiter; Guarded kennzeichnet nur lokale oder ungeprüfte Relay-Routen.",
    "Provider-Ausgabeschutz deaktivieren?\nFastCtx folgt weiterhin der gewählten Stufe, kann aber Usage, Komprimierung, Katalog und Kontextfehler des Providers nicht prüfen. Dies betrifft Apply und neue Laufzeitsitzungen."
);
const PT_BR: GuardMessages = guard_messages!(
    "Compatibilidade do provider",
    "Proteção do provider",
    "Este provider usa compactação local: o Codex reenvia todo o histórico ao modelo. O Guarded mantém o nível escolhido e os orçamentos individuais; não aplica um pool compartilhado por turno e não verifica o catálogo nem o limite de contexto.",
    "Este provider é um relay cujo protocolo de compactação e catálogo de modelos o FastCtx não consegue verificar. O Guarded mantém o nível escolhido e os orçamentos individuais; não aplica um pool compartilhado por turno nem substitui o tratamento correto do relay.",
    "A proteção está desativada. O FastCtx continua seguindo o nível escolhido, mas não verifica catálogo, usage, compactação ou erros de contexto do provider.",
    "A proteção está ativa, mas ociosa: esta rota é OpenAI oficial, Azure ou Bedrock. O nível escolhido continua valendo; o Guarded apenas identifica rotas locais ou relays não verificados.",
    "Desativar a proteção de saída do provider?\nO FastCtx continuará seguindo o nível escolhido, mas não verifica usage, compactação, catálogo ou erros de contexto do provider. Isso afeta Apply e novas sessões."
);
const RU: GuardMessages = guard_messages!(
    "Совместимость provider",
    "Защита provider",
    "Этот provider использует локальное сжатие: Codex заново передаёт модели всю историю. Guarded сохраняет выбранный уровень и отдельные бюджеты инструментов; общего пула на ход нет, а каталог модели не проверяется.",
    "Этот provider — посредник, протокол сжатия и каталог моделей которого FastCtx не может проверить. Guarded сохраняет выбранный уровень и отдельные бюджеты инструментов; общего пула на ход нет и он не заменяет корректную обработку посредника.",
    "Защита отключена. FastCtx продолжает использовать выбранный уровень, но не может проверить каталог, usage, сжатие и ошибки контекста provider.",
    "Защита включена, но сейчас бездействует: это официальный маршрут OpenAI, Azure или Bedrock. Выбранный уровень действует; Guarded лишь отмечает локальные и непроверенные маршруты.",
    "Отключить защиту вывода provider?\nFastCtx продолжит использовать выбранный уровень, но не может проверить usage, сжатие, каталог и ошибки контекста provider. Это влияет на Apply и новые сеансы."
);
const IT: GuardMessages = guard_messages!(
    "Compatibilità provider",
    "Protezione provider",
    "Questo provider usa la compattazione locale: Codex rimanda l’intera cronologia al modello. Guarded conserva il livello scelto e i budget individuali; non applica un pool condiviso per turno e non verifica catalogo o soglia di contesto.",
    "Questo provider è un relay di cui FastCtx non può verificare protocollo di compattazione e catalogo modelli. Guarded conserva il livello scelto e i budget individuali; non applica un pool condiviso per turno e non sostituisce la corretta gestione del relay.",
    "La protezione è disattivata. FastCtx segue il livello scelto, ma non può verificare catalogo, usage, compattazione ed errori di contesto del provider.",
    "La protezione è attiva ma a riposo: questa rotta è OpenAI ufficiale, Azure o Bedrock. Resta valido il livello scelto; Guarded segnala soltanto rotte locali o relay non verificati.",
    "Disattivare la protezione output del provider?\nFastCtx seguirà il livello scelto, ma non può verificare usage, compattazione, catalogo ed errori di contesto del provider. Riguarda Apply e le nuove sessioni."
);
const TR: GuardMessages = guard_messages!(
    "Provider uyumluluğu",
    "Provider koruması",
    "Bu provider yerel sıkıştırma kullanır; Codex tüm geçmişi modele yeniden gönderir. Guarded seçilen kademeyi ve tek tek araç bütçelerini korur; tur başına ortak havuz uygulamaz ve model kataloğunu doğrulayamaz.",
    "Bu provider, sıkıştırma protokolü ve model kataloğu FastCtx tarafından doğrulanamayan bir aktarıcıdır. Guarded seçilen kademeyi ve tek tek araç bütçelerini korur; tur başına ortak havuz uygulamaz ve aktarıcının işlemlerinin yerini tutmaz.",
    "Koruma kapalı. FastCtx seçilen kademeyi izler, ancak provider’ın katalog, usage, sıkıştırma ve bağlam hatası sözleşmesini doğrulayamaz.",
    "Koruma açık ama şu an devrede değil: bu rota resmi OpenAI, Azure veya Bedrock’tır. Seçilen kademe geçerlidir; Guarded yalnızca yerel veya doğrulanmamış aktarıcı rotalarını belirtir.",
    "Provider çıktı koruması kapatılsın mı?\nFastCtx seçilen kademeyi izler, ancak provider’ın usage, sıkıştırma, katalog ve bağlam hatası işlemesini doğrulayamaz. Apply ve yeni çalışma zamanı oturumlarını etkiler."
);
const PL: GuardMessages = guard_messages!(
    "Zgodność providera",
    "Ochrona providera",
    "Ten provider używa lokalnej kompakcji: Codex ponownie przekazuje modelowi całą historię. Guarded zachowuje wybrany poziom i budżety poszczególnych narzędzi; nie stosuje wspólnej puli na turę i nie weryfikuje katalogu modelu.",
    "Ten provider jest pośrednikiem, którego protokołu kompakcji i katalogu modeli FastCtx nie może zweryfikować. Guarded zachowuje wybrany poziom i budżety poszczególnych narzędzi; nie stosuje wspólnej puli na turę i nie zastępuje poprawnej obsługi pośrednika.",
    "Ochrona jest wyłączona. FastCtx nadal używa wybranego poziomu, ale nie może zweryfikować katalogu, usage, kompakcji ani błędów kontekstu providera.",
    "Ochrona jest włączona, ale bezczynna: to oficjalna trasa OpenAI, Azure lub Bedrock. Wybrany poziom obowiązuje; Guarded tylko oznacza lokalne i niezweryfikowane trasy.",
    "Wyłączyć ochronę wyjścia providera?\nFastCtx nadal będzie używać wybranego poziomu, ale nie może zweryfikować usage, kompakcji, каталогu ani błędów kontekstu providera. Dotyczy Apply i nowych sesji."
);
const NL: GuardMessages = guard_messages!(
    "Providercompatibiliteit",
    "Providerbescherming",
    "Deze provider gebruikt lokale compactie: Codex voert de hele geschiedenis opnieuw aan het model. Guarded behoudt het gekozen niveau en de afzonderlijke toolbudgetten; er is geen gedeelde pool per beurt en de modelcatalogus blijft onverified.",
    "Deze provider is een relay waarvan FastCtx het compactieprotocol en de modelcatalogus niet kan verifiëren. Guarded behoudt het gekozen niveau en de afzonderlijke toolbudgetten; er is geen gedeelde pool per beurt en het vervangt geen correcte relayafhandeling.",
    "Bescherming is uitgeschakeld. FastCtx volgt het gekozen niveau, maar kan catalogus, usage, compactie en contextfouten van de provider niet verifiëren.",
    "Bescherming is ingeschakeld maar rust: dit is een officiële OpenAI-, Azure- of Bedrock-route. Het gekozen niveau blijft gelden; Guarded markeert alleen lokale of niet-geverifieerde relayroutes.",
    "Provider-uitvoerbescherming uitschakelen?\nFastCtx blijft het gekozen niveau volgen, maar kan usage, compactie, catalogus en contextfouten van de provider niet verifiëren. Dit raakt Apply en nieuwe runtimesessies."
);
const VI: GuardMessages = guard_messages!(
    "Tương thích provider",
    "Bảo vệ provider",
    "Provider này dùng nén cục bộ: Codex gửi lại toàn bộ lịch sử cho mô hình. Guarded giữ mức đã chọn và ngân sách riêng của từng công cụ; không áp dụng nhóm dùng chung mỗi lượt và không thể xác minh danh mục mô hình.",
    "Provider này là relay có giao thức nén và danh mục mô hình mà FastCtx không thể xác minh. Guarded giữ mức đã chọn và ngân sách riêng của từng công cụ; không áp dụng nhóm dùng chung mỗi lượt và không thay thế xử lý đúng của relay.",
    "Bảo vệ đã tắt. FastCtx vẫn theo mức đã chọn, nhưng không thể xác minh danh mục, usage, nén và lỗi ngữ cảnh của provider.",
    "Bảo vệ đang bật nhưng chưa cần dùng: đây là tuyến OpenAI chính thức, Azure hoặc Bedrock. Mức đã chọn vẫn có hiệu lực; Guarded chỉ đánh dấu tuyến cục bộ hoặc relay chưa xác minh.",
    "Tắt bảo vệ đầu ra provider?\nFastCtx vẫn theo mức đã chọn, nhưng không thể xác minh usage, nén, danh mục và xử lý lỗi ngữ cảnh của provider. Ảnh hưởng đến Apply và các phiên chạy mới."
);
const ID: GuardMessages = guard_messages!(
    "Kompatibilitas provider",
    "Perlindungan provider",
    "Provider ini memakai kompaksi lokal: Codex mengirim ulang seluruh riwayat ke model. Guarded mempertahankan tingkat pilihan dan anggaran tiap alat; tidak memakai pool bersama per giliran dan tidak dapat memverifikasi katalog model.",
    "Provider ini adalah relay yang protokol kompaksi dan katalog modelnya tidak dapat diverifikasi FastCtx. Guarded mempertahankan tingkat pilihan dan anggaran tiap alat; tidak memakai pool bersama per giliran dan tidak menggantikan penanganan relay yang benar.",
    "Perlindungan dinonaktifkan. FastCtx tetap mengikuti tingkat pilihan, tetapi tidak dapat memverifikasi katalog, usage, kompaksi, dan galat konteks provider.",
    "Perlindungan aktif tetapi belum diperlukan: ini rute OpenAI resmi, Azure, atau Bedrock. Tingkat pilihan tetap berlaku; Guarded hanya menandai rute lokal atau relay yang belum diverifikasi.",
    "Nonaktifkan perlindungan keluaran provider?\nFastCtx tetap mengikuti tingkat pilihan, tetapi tidak dapat memverifikasi usage, kompaksi, katalog, dan galat konteks provider. Ini memengaruhi Apply dan sesi runtime baru."
);
const UK: GuardMessages = guard_messages!(
    "Сумісність provider",
    "Захист provider",
    "Цей provider використовує локальне стиснення: Codex знову передає моделі всю історію. Guarded зберігає вибраний рівень і окремі бюджети інструментів; спільний пул за хід не застосовується, а каталог моделі не перевіряється.",
    "Цей provider є посередником, протокол стиснення й каталог моделей якого FastCtx не може перевірити. Guarded зберігає вибраний рівень і окремі бюджети інструментів; спільний пул за хід не застосовується й не замінює правильну обробку посередника.",
    "Захист вимкнено. FastCtx дотримується вибраного рівня, але не може перевірити каталог, usage, стиснення та помилки контексту provider.",
    "Захист увімкнено, але наразі бездіяльний: це офіційний маршрут OpenAI, Azure або Bedrock. Вибраний рівень діє; Guarded лише позначає локальні й неперевірені маршрути.",
    "Вимкнути захист виводу provider?\nFastCtx дотримуватиметься вибраного рівня, але не може перевірити usage, стиснення, каталог і помилки контексту provider. Це впливає на Apply і нові сеанси."
);

#[cfg(test)]
mod tests {
    use super::messages;
    use crate::control::i18n::ALL_LANGUAGES;

    #[test]
    fn every_language_has_complete_provider_guard_copy() {
        for language in ALL_LANGUAGES {
            let messages = messages(language);
            assert!(
                messages
                    .values()
                    .iter()
                    .all(|value| !value.trim().is_empty()),
                "{} has an empty provider-guard translation",
                language.code()
            );
            // The section header sits directly above the item row, so identical text
            // renders as a duplicated line rather than a heading and its setting.
            assert_ne!(
                messages.section_title,
                messages.label,
                "{} repeats the provider-guard label as its section header",
                language.code()
            );
            assert_ne!(
                messages.active_note,
                messages.relay_active_note,
                "{} does not distinguish local compaction from an unverified relay",
                language.code()
            );
            for value in messages.values() {
                assert!(!value.contains("10000"));
                assert!(!value.contains("9000"));
            }
        }
    }
}

//! Static daily text entries for reusable Vaachak apps.

use crate::vaachak_x4::text::{ScriptClass, dominant_script};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub const fn from_monday_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Monday),
            1 => Some(Self::Tuesday),
            2 => Some(Self::Wednesday),
            3 => Some(Self::Thursday),
            4 => Some(Self::Friday),
            5 => Some(Self::Saturday),
            6 => Some(Self::Sunday),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DailyScheduleKey {
    Weekday(Weekday),
    MonthDay(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DailyTextEntry {
    pub key: DailyScheduleKey,
    pub title: &'static str,
    pub dedication: &'static str,
    pub sanskrit: &'static str,
    pub hindi: &'static str,
    pub english: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DailyTextScriptSummary {
    pub title_script: ScriptClass,
    pub sanskrit_script: ScriptClass,
    pub hindi_script: ScriptClass,
    pub english_script: ScriptClass,
}

impl DailyTextEntry {
    pub fn script_summary(self) -> DailyTextScriptSummary {
        DailyTextScriptSummary {
            title_script: dominant_script(self.title),
            sanskrit_script: dominant_script(self.sanskrit),
            hindi_script: dominant_script(self.hindi),
            english_script: dominant_script(self.english),
        }
    }
}

pub const DAILY_HINDU_MANTRAS: [DailyTextEntry; 7] = [
    DailyTextEntry {
        key: DailyScheduleKey::Weekday(Weekday::Monday),
        title: "Monday — Somvar",
        dedication: "Dedicated to Lord Shiva",
        sanskrit: "ॐ नमः शिवाय",
        hindi: "ओम नमः शिवाय (भगवान शिव को नमस्कार)",
        english: "Om Namah Shivaya (Adoration to Lord Shiva)",
    },
    DailyTextEntry {
        key: DailyScheduleKey::Weekday(Weekday::Tuesday),
        title: "Tuesday — Mangalvar",
        dedication: "Dedicated to Lord Hanuman and Lord Ganesha",
        sanskrit: "ॐ हनुमते नमः",
        hindi: "ओम हनुमते नमः (भगवान हनुमान को नमस्कार)",
        english: "Om Hanumate Namah (Salutations to Lord Hanuman)",
    },
    DailyTextEntry {
        key: DailyScheduleKey::Weekday(Weekday::Wednesday),
        title: "Wednesday — Budhvar",
        dedication: "Dedicated to Lord Ganesha and Lord Krishna",
        sanskrit: "ॐ गं गणपतये नमः",
        hindi: "ओम गं गणपतये नमः (भगवान गणेश को नमस्कार)",
        english: "Om Gam Ganapataye Namah (Salutations to Lord Ganesha)",
    },
    DailyTextEntry {
        key: DailyScheduleKey::Weekday(Weekday::Thursday),
        title: "Thursday — Guruvar",
        dedication: "Dedicated to Lord Vishnu and Brihaspati",
        sanskrit: "ॐ नमो भगवते वासुदेवाय",
        hindi: "ओम नमो भगवते वासुदेवाय (भगवान विष्णु को मेरा प्रणाम)",
        english: "Om Namo Bhagavate Vasudevaya (I bow to Lord Vasudeva/Vishnu)",
    },
    DailyTextEntry {
        key: DailyScheduleKey::Weekday(Weekday::Friday),
        title: "Friday — Shukravar",
        dedication: "Dedicated to Goddess Lakshmi and Devi",
        sanskrit: "ॐ श्रीं महालक्ष्म्यै नमः",
        hindi: "ओम श्रीं महालक्ष्म्यै नमः (देवी महालक्ष्मी को नमस्कार)",
        english: "Om Shreem Mahalakshmyai Namah (Salutations to Goddess Mahalakshmi)",
    },
    DailyTextEntry {
        key: DailyScheduleKey::Weekday(Weekday::Saturday),
        title: "Saturday — Shanivar",
        dedication: "Dedicated to Lord Shani",
        sanskrit: "ॐ शनैश्चराय नमः",
        hindi: "ओम शनैश्चराय नमः (भगवान शनिदेव को नमस्कार)",
        english: "Om Shanaishcharaya Namah (Salutations to Lord Shani)",
    },
    DailyTextEntry {
        key: DailyScheduleKey::Weekday(Weekday::Sunday),
        title: "Sunday — Ravivar",
        dedication: "Dedicated to Surya Dev (The Sun God)",
        sanskrit: "ॐ सूर्याय नमः",
        hindi: "ओम सूर्याय नमः (सूर्य देव को नमस्कार)",
        english: "Om Suryaya Namah (Salutations to the Sun God)",
    },
];

pub fn entry_for_weekday(day: Weekday) -> &'static DailyTextEntry {
    for entry in DAILY_HINDU_MANTRAS.iter() {
        if entry.key == DailyScheduleKey::Weekday(day) {
            return entry;
        }
    }
    &DAILY_HINDU_MANTRAS[0]
}

pub fn entry_for_month_day(day: u8) -> &'static DailyTextEntry {
    if day == 0 {
        return &DAILY_HINDU_MANTRAS[0];
    }
    let index = ((day - 1) as usize) % DAILY_HINDU_MANTRAS.len();
    &DAILY_HINDU_MANTRAS[index]
}

#[cfg(test)]
mod tests {
    use super::{Weekday, entry_for_month_day, entry_for_weekday};
    use crate::vaachak_x4::text::ScriptClass;

    #[test]
    fn monday_entry_uses_devanagari_and_latin_scripts() {
        let entry = entry_for_weekday(Weekday::Monday);
        let summary = entry.script_summary();
        assert_eq!(summary.sanskrit_script, ScriptClass::Devanagari);
        assert_eq!(summary.hindi_script, ScriptClass::Devanagari);
        assert_eq!(summary.english_script, ScriptClass::Latin);
    }

    #[test]
    fn month_day_wraps_static_entries() {
        assert_eq!(entry_for_month_day(1).title, "Monday — Somvar");
        assert_eq!(entry_for_month_day(8).title, "Monday — Somvar");
    }
}

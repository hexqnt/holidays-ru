use crate::raw_date::RawDate;
use crate::{DayFlags, Resolved, official, predict_mod};

mod sealed {
    pub trait Sealed {}
}

/// Типизированный источник календарных данных.
///
/// Реализации этого trait предоставляются крейтом. Пользователь выбирает
/// календарь через marker-тип, например [`Federal`] или [`regions::Tatarstan`].
pub trait Calendar: sealed::Sealed {
    /// `true`, если календарь содержит полную информацию о рабочих и нерабочих днях.
    const IS_COMPLETE: bool;

    /// Возвращает [`DayFlags`] для даты, заданной годом, месяцем и днём.
    fn flags_ymd(year: i32, month: u8, day: u8) -> Option<Resolved<DayFlags>>;
}

/// Федеральный производственный календарь РФ.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Federal;

impl sealed::Sealed for Federal {}

impl Calendar for Federal {
    const IS_COMPLETE: bool = true;

    #[inline]
    fn flags_ymd(year: i32, month: u8, day: u8) -> Option<Resolved<DayFlags>> {
        RawDate::from_ymd(year, month, day).map(federal_flags_raw)
    }
}

#[inline]
pub(crate) fn federal_flags_raw(date: RawDate) -> Resolved<DayFlags> {
    if let Some(flags) = official::flags(date) {
        Resolved::Fact(flags)
    } else {
        Resolved::Predict(predict_mod::flags(date))
    }
}

macro_rules! region_marker {
    ($name:ident, $module:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl super::sealed::Sealed for $name {}

        impl super::Calendar for $name {
            const IS_COMPLETE: bool = false;

            #[inline]
            fn flags_ymd(
                year: i32,
                month: u8,
                day: u8,
            ) -> Option<crate::Resolved<crate::DayFlags>> {
                let date = crate::raw_date::RawDate::from_ymd(year, month, day)?;
                Some(crate::data::regional::$module::flags(date))
            }
        }
    };
}

/// Marker-типы региональных overlay-календарей.
pub mod regions {
    region_marker!(Adygea, adygea, "Республика Адыгея.");
    region_marker!(AltaiRepublic, altai_republic, "Республика Алтай.");
    region_marker!(Bashkortostan, bashkortostan, "Республика Башкортостан.");
    region_marker!(Buryatia, buryatia, "Республика Бурятия.");
    region_marker!(Dagestan, dagestan, "Республика Дагестан.");
    region_marker!(Ingushetia, ingushetia, "Республика Ингушетия.");
    region_marker!(
        KabardinoBalkaria,
        kabardino_balkaria,
        "Кабардино-Балкарская Республика."
    );
    region_marker!(Kalmykia, kalmykia, "Республика Калмыкия.");
    region_marker!(
        KarachayCherkessia,
        karachay_cherkessia,
        "Карачаево-Черкесская Республика."
    );
    region_marker!(Crimea, crimea, "Республика Крым.");
    region_marker!(Mordovia, mordovia, "Республика Мордовия.");
    region_marker!(
        NorthOssetiaAlania,
        north_ossetia_alania,
        "Республика Северная Осетия - Алания."
    );
    region_marker!(Tatarstan, tatarstan, "Республика Татарстан.");
    region_marker!(Tuva, tuva, "Республика Тыва.");
    region_marker!(Chechnya, chechnya, "Чеченская Республика.");
    region_marker!(Chuvashia, chuvashia, "Чувашская Республика - Чувашия.");
    region_marker!(ZabaykalskyKrai, zabaykalsky_krai, "Забайкальский край.");
    region_marker!(KrasnodarKrai, krasnodar_krai, "Краснодарский край.");
    region_marker!(StavropolKrai, stavropol_krai, "Ставропольский край.");
    region_marker!(BelgorodOblast, belgorod_oblast, "Белгородская область.");
    region_marker!(BryanskOblast, bryansk_oblast, "Брянская область.");
    region_marker!(IrkutskOblast, irkutsk_oblast, "Иркутская область.");
    region_marker!(OmskOblast, omsk_oblast, "Омская область.");
    region_marker!(PenzaOblast, penza_oblast, "Пензенская область.");
    region_marker!(SaratovOblast, saratov_oblast, "Саратовская область.");
}

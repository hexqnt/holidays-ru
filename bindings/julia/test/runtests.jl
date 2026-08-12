using Dates: Date
using HolidaysRu
using Test

@testset "HolidaysRu" begin
    @testset "federal days" begin
        info = day_info(Date(2026, 1, 9))
        @test is_day_off(info)
        @test is_transferred(info)
        @test !is_holiday(info)
        @test is_official(info)
        @test is_day_off(Date(2026, 1, 9))
        @test is_transferred(info.flags)

        predicted = day_info(Date(2027, 1, 1))
        @test is_holiday(predicted)
        @test is_predicted(predicted)
    end

    @testset "regional calendar" begin
        @test length(instances(Region)) == HolidaysRu._REGION_COUNT

        federal = day_info(Date(2026, 11, 6))
        tatarstan = day_info(Date(2026, 11, 6); region=Tatarstan)

        @test is_working_day(federal)
        @test is_day_off(tatarstan)
        @test is_holiday(tatarstan)

        calendar = Calendar(Tatarstan)
        @test is_day_off(calendar, Date(2026, 11, 6))
        @test is_working_day(Calendar(), Date(2026, 11, 6))
        @test non_working_days_between(
            calendar,
            Date(2026, 11, 6),
            Date(2026, 11, 7),
        ).value == 1

        for region in instances(Region)
            @test day_info(Date(2026, 1, 12); region).date == Date(2026, 1, 12)
        end
    end

    @testset "ranges" begin
        days = non_working_days_between(Date(2026, 1, 1), Date(2027, 1, 1))
        @test days.value == 118
        @test is_official(days)

        minutes = working_minutes_between(
            Date(2026, 1, 12),
            Date(2026, 1, 13);
            week=ThirtySixHours,
        )
        @test minutes.value == 7 * 60 + 12
        @test working_hours_between(
            Date(2026, 1, 12),
            Date(2026, 1, 13);
            week=ThirtySixHours,
        ).value == 7.2

        calendar = Calendar()
        hours = working_hours_between(
            calendar,
            Date(2026, 1, 12),
            Date(2026, 1, 13);
            week=TwentyFourHours,
        )
        @test hours.value == 4.8
        @test is_official(hours)
    end

    @testset "validation and constants" begin
        @test isempty(detect_ambiguities(HolidaysRu))
        @test FIRST_FACT_YEAR == 1993
        @test LAST_FACT_YEAR == 2026
        @test MIN_YEAR == 1900
        @test MAX_YEAR == 2100
        @test UInt8(FortyHours) == 40
        @test UInt8(ThirtySixHours) == 36
        @test UInt8(TwentyFourHours) == 24
        @test_throws ArgumentError day_info(Date(1899, 12, 31))
        @test_throws ArgumentError non_working_days_between(
            Date(2026, 1, 2),
            Date(2026, 1, 1),
        )
        last_supported_day = Date(MAX_YEAR, 12, 31)
        last_day_range = non_working_days_between(
            last_supported_day,
            Date(MAX_YEAR + 1, 1, 1),
        )
        @test last_day_range.value == Int(is_day_off(last_supported_day))
    end
end

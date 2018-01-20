defmodule Meetup do
  @moduledoc """
  Calculate meetup dates.
  """

  @type weekday ::
          :monday
          | :tuesday
          | :wednesday
          | :thursday
          | :friday
          | :saturday
          | :sunday

  @type schedule :: :first | :second | :third | :fourth | :last | :teenth

  @doc """
  Calculate a meetup date.

  The schedule is in which week (1..4, last or "teenth") the meetup date should
  fall.
  """
  @spec meetup(pos_integer, pos_integer, weekday, schedule) :: :calendar.date()
  def meetup(year, month, weekday, schedule) do
    meetup_impl(year, month, weekday, schedule)
  end

  defp meetup_impl(year, month, weekday, :first), do: find_date(year, month, 1..7, weekday)
  defp meetup_impl(year, month, weekday, :second), do: find_date(year, month, 8..14, weekday)
  defp meetup_impl(year, month, weekday, :third), do: find_date(year, month, 15..21, weekday)
  defp meetup_impl(year, month, weekday, :fourth), do: find_date(year, month, 22..28, weekday)
  defp meetup_impl(year, month, weekday, :teenth), do: find_date(year, month, 13..19, weekday)

  defp meetup_impl(year, month, weekday, :last) do
    days_in_month = Date.days_in_month(Date.from_erl!({year, month, 1}))
    range = Range.new(days_in_month - 6, days_in_month)
    find_date(year, month, range, weekday)
  end

  defp find_date(year, month, range, weekday) do
    range
    |> Enum.map(&Date.from_erl!({year, month, &1}))
    |> Enum.find(&is_weekday(&1, weekday))
    |> Date.to_erl()
  end

  @week [nil, :monday, :tuesday, :wednesday, :thursday, :friday, :saturday, :sunday]
  defp is_weekday(date, weekday) do
    Date.day_of_week(date) == Enum.find_index(@week, &(&1 == weekday))
  end
end

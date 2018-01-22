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

  @day_of_week %{
    :monday => 1,
    :tuesday => 2,
    :wednesday => 3,
    :thursday => 4,
    :friday => 5,
    :saturday => 6,
    :sunday => 7
  }

  @doc """
  Calculate a meetup date.

  The schedule is in which week (1..4, last or "teenth") the meetup date should
  fall.
  """
  @spec meetup(pos_integer, pos_integer, weekday, schedule) :: :calendar.date()
  def meetup(year, month, weekday, schedule) do
    meetup_impl(year, month, weekday, schedule)
  end

  defp meetup_impl(year, month, weekday, :first), do: find_date(year, month, 1, weekday)
  defp meetup_impl(year, month, weekday, :second), do: find_date(year, month, 8, weekday)
  defp meetup_impl(year, month, weekday, :third), do: find_date(year, month, 15, weekday)
  defp meetup_impl(year, month, weekday, :fourth), do: find_date(year, month, 22, weekday)
  defp meetup_impl(year, month, weekday, :teenth), do: find_date(year, month, 13, weekday)

  defp meetup_impl(year, month, weekday, :last) do
    find_date(year, month, Date.days_in_month(Date.from_erl!({year, month, 1})) - 6, weekday)
  end

  defp find_date(year, month, start, weekday) do
    Range.new(start, start + 6)
    |> Stream.map(&Date.from_erl!({year, month, &1}))
    |> Enum.find(&(Date.day_of_week(&1) == @day_of_week[weekday]))
    |> Date.to_erl()
  end
end

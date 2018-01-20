defmodule Year do
  @doc """
  Returns whether 'year' is a leap year.

  A leap year occurs:

  on every year that is evenly divisible by 4
    except every year that is evenly divisible by 100
      unless the year is also evenly divisible by 400
  """
  @spec leap_year?(non_neg_integer) :: boolean
  def leap_year?(year) do
    leap_year_impl(year)
  end

  defp leap_year_impl(y) when rem(y, 400) == 0, do: true
  defp leap_year_impl(y) when rem(y, 100) == 0, do: false
  defp leap_year_impl(y) when rem(y, 4) == 0, do: true
  defp leap_year_impl(_), do: false
end

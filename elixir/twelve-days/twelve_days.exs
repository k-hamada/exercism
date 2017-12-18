defmodule TwelveDays do
  @doc """
  Given a `number`, return the song's verse for that specific day, including
  all gifts for previous days in the same line.
  """
  @spec verse(number :: integer) :: String.t()
  def verse(number) do
    "#{day(number)}, #{presents(number)}."
  end

  @doc """
  Given a `starting_verse` and an `ending_verse`, return the verses for each
  included day, one per line.
  """
  @spec verses(starting_verse :: integer, ending_verse :: integer) :: String.t()
  def verses(starting_verse, ending_verse) do
    starting_verse..ending_verse
    |> Enum.map(&verse/1)
    |> Enum.join("\n")
  end

  @doc """
  Sing all 12 verses, in order, one verse per line.
  """
  @spec sing():: String.t()
  def sing do
    verses(1, 12)
  end


  defp day(n) do
    "On the #{ordinal(n)} day of Christmas my true love gave to me"
  end

  defp ordinal(n) when n ==  1 do "first" end
  defp ordinal(n) when n ==  2 do "second" end
  defp ordinal(n) when n ==  3 do "third" end
  defp ordinal(n) when n ==  4 do "fourth" end
  defp ordinal(n) when n ==  5 do "fifth" end
  defp ordinal(n) when n ==  6 do "sixth" end
  defp ordinal(n) when n ==  7 do "seventh" end
  defp ordinal(n) when n ==  8 do "eighth" end
  defp ordinal(n) when n ==  9 do "ninth" end
  defp ordinal(n) when n == 10 do "tenth" end
  defp ordinal(n) when n == 11 do "eleventh" end
  defp ordinal(n) when n == 12 do "twelfth" end

  defp presents(n) do
    Enum.to_list(n..1)
    |> presents([])
    |> Enum.reverse
    |> Enum.join(", ")
  end

  defp presents([1], []) do
    [present(1)]
  end

  defp presents([1], lst) do
    ["and #{present(1)}" | lst]
  end

  defp presents([h | t], lst) do
    presents(t, [present(h) | lst])
  end

  defp present(n) when n ==  1 do "a Partridge in a Pear Tree" end
  defp present(n) when n ==  2 do "two Turtle Doves" end
  defp present(n) when n ==  3 do "three French Hens" end
  defp present(n) when n ==  4 do "four Calling Birds" end
  defp present(n) when n ==  5 do "five Gold Rings" end
  defp present(n) when n ==  6 do "six Geese-a-Laying" end
  defp present(n) when n ==  7 do "seven Swans-a-Swimming" end
  defp present(n) when n ==  8 do "eight Maids-a-Milking" end
  defp present(n) when n ==  9 do "nine Ladies Dancing" end
  defp present(n) when n == 10 do "ten Lords-a-Leaping" end
  defp present(n) when n == 11 do "eleven Pipers Piping" end
  defp present(n) when n == 12 do "twelve Drummers Drumming" end
end

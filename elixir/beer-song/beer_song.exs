defmodule BeerSong do
  @doc """
  Get a single verse of the beer song
  """
  @spec verse(integer) :: String.t
  def verse(number) when number in 99..0 do
    """
    #{String.capitalize(bottle(number))} of beer on the wall, #{bottle(number)} of beer.
    #{take(number)}, #{bottle(number - 1)} of beer on the wall.
    """
  end

  defp bottle(-1), do: 99 |> bottle
  defp bottle(0), do: "no more" |> bottle
  defp bottle(1), do: "1 bottle"
  defp bottle(number), do: "#{number} bottles"

  defp take(0), do: "Go to the store and buy some more"
  defp take(1), do: 1 |> take("it")
  defp take(_, it \\ "one"), do: "Take #{it} down and pass it around"

  @doc """
  Get the entire beer song for a given range of numbers of bottles.
  """
  @spec lyrics(Range.t) :: String.t
  def lyrics(range \\ 99..0) do
    range
    |> Enum.map_join("\n", &verse/1)
  end
end

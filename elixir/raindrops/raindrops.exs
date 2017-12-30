defmodule Raindrops do
  @doc """
  Returns a string based on raindrop factors.

  - If the number contains 3 as a prime factor, output 'Pling'.
  - If the number contains 5 as a prime factor, output 'Plang'.
  - If the number contains 7 as a prime factor, output 'Plong'.
  - If the number does not contain 3, 5, or 7 as a prime factor,
    just pass the number's digits straight through.
  """
  @spec convert(pos_integer) :: String.t
  def convert(number) do
    [
      raindrop(number, 3, "Pling"),
      raindrop(number, 5, "Plang"),
      raindrop(number, 7, "Plong"),
    ]
    |> Enum.join
    |> output(to_string(number))
  end

  defp raindrop(number, factor, word) do
    case rem(number, factor) do
      0 -> word
      _ -> ""
    end
  end

  defp output("", through), do: through
  defp output(raindrops, _), do: raindrops
end

defmodule Bob do
  def hey(input) do
    cond do
      say?(input)  -> "Fine. Be that way!"
      ask?(input)  -> "Sure."
      yell?(input) -> "Whoa, chill out!"
      true         -> "Whatever."
    end
  end

  defp say?(input) do
    String.trim(input) == ""
  end

  defp ask?(input) do
    String.ends_with?(input, "?")
  end

  defp yell?(input) do
    words = String.split(input, ~r/[[:digit:][:punct:][:space:]]/, trim: true)
    unless Enum.empty?(words) do
      words |> Enum.all?(&(&1 == String.upcase(&1)))
    else
      false
    end
  end
end

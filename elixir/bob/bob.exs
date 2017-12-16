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
    input == String.upcase(input) && input != String.downcase(input)
  end
end

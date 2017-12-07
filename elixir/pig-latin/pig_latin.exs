defmodule PigLatin do
  @doc """
  Given a `phrase`, translate it a word at a time to Pig Latin.

  Words beginning with consonants should have the consonant moved to the end of
  the word, followed by "ay".

  Words beginning with vowels (aeiou) should have "ay" added to the end of the
  word.

  Some groups of letters are treated like consonants, including "ch", "qu",
  "squ", "th", "thr", and "sch".

  Some groups are treated like vowels, including "yt" and "xr".
  """
  @spec translate(phrase :: String.t()) :: String.t()
  def translate(phrase) do
    phrase
    |> String.split( " ")
    |> Enum.map(&translate_word/1)
    |> Enum.join(" ")
  end

  defp translate_word(word) do
    matcher = ~r{a|i|u(?!a|i|e|o)|e|o|yt|xr}
    pigword =
      unless String.first(word) |> String.match?(matcher) do
        [x, y, z] = String.split(word, matcher, parts: 2, include_captures: true)
        y <> z <> x
      else
        word
      end
    pigword <> "ay"
  end
end

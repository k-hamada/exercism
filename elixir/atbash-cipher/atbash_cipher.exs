defmodule Atbash do
  @doc """
  Encode a given plaintext to the corresponding ciphertext

  ## Examples

  iex> Atbash.encode("completely insecure")
  "xlnko vgvob rmhvx fiv"
  """
  @spec encode(String.t()) :: String.t()
  def encode(plaintext) do
    to_cipher_codepoints(plaintext)
    |> Enum.chunk_every(5)
    |> Enum.map(&to_string(&1))
    |> Enum.join(" ")
  end

  @spec decode(String.t()) :: String.t()
  def decode(cipher) do
    to_cipher_codepoints(cipher)
    |> to_string
  end

  defp to_cipher_codepoints(text) do
    String.downcase(text)
    |> String.codepoints()
    |> Enum.reject(&(&1 == " "))
    |> Enum.map(&to_codepoint(&1))
    |> Enum.map(&do_cipher(&1))
    |> Enum.reject(&is_nil/1)
  end

  defp to_codepoint(char) do
    <<codepoint::utf8>> = char
    codepoint
  end

  defp do_cipher(codepoint) when codepoint in 97..122 do
    25 - (codepoint - 97) + 97
  end

  defp do_cipher(codepoint) when codepoint in 48..59, do: codepoint
  defp do_cipher(codepoint) when codepoint in 65..90, do: codepoint
  defp do_cipher(32), do: 32
  defp do_cipher(_), do: nil
end

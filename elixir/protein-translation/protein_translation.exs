defmodule ProteinTranslation do
  @codon_map %{
    "UGU" => "Cysteine",
    "UGC" => "Cysteine",
    "UUA" => "Leucine",
    "UUG" => "Leucine",
    "AUG" => "Methionine",
    "UUU" => "Phenylalanine",
    "UUC" => "Phenylalanine",
    "UCU" => "Serine",
    "UCC" => "Serine",
    "UCA" => "Serine",
    "UCG" => "Serine",
    "UGG" => "Tryptophan",
    "UAU" => "Tyrosine",
    "UAC" => "Tyrosine",
    "UAA" => "STOP",
    "UAG" => "STOP",
    "UGA" => "STOP"
  }

  @doc """
  Given an RNA string, return a list of proteins specified by codons, in order.
  """
  @spec of_rna(String.t()) :: { atom,  list(String.t()) }
  def of_rna(rna) do
    rna
    |> to_codons
    |> to_rna
  end

  defp to_codons(rna) do
    String.to_charlist(rna)
    |> Enum.chunk_every(3, 3, :discard)
    |> Enum.map(&List.to_string/1)
  end

  defp to_rna(codons) do
    to_rna(codons, [])
  end

  defp to_rna([head | tail], proteins) do
    case of_codon(head) do
      { :error, _ } -> { :error, "invalid RNA" }
      { :ok, "STOP" } -> to_rna([], proteins)
      { :ok, protein } -> to_rna(tail, proteins ++ [protein])
    end
  end

  defp to_rna([], proteins) do
    { :ok, proteins }
  end

  @doc """
  Given a codon, return the corresponding protein

  UGU -> Cysteine
  UGC -> Cysteine
  UUA -> Leucine
  UUG -> Leucine
  AUG -> Methionine
  UUU -> Phenylalanine
  UUC -> Phenylalanine
  UCU -> Serine
  UCC -> Serine
  UCA -> Serine
  UCG -> Serine
  UGG -> Tryptophan
  UAU -> Tyrosine
  UAC -> Tyrosine
  UAA -> STOP
  UAG -> STOP
  UGA -> STOP
  """
  @spec of_codon(String.t()) :: { atom, String.t() }
  def of_codon(codon) do
    case Map.fetch(@codon_map, codon) do
      :error -> { :error, "invalid codon" }
      value -> value
    end
  end
end

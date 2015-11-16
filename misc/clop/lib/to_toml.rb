def to_toml(hash)
  hash.map do |namespace, fields|
    str = "[#{namespace}]\n"
    str += fields.map do |key, value|
      "#{key} = #{value}"
    end.join("\n")
  end.join("\n\n")
end

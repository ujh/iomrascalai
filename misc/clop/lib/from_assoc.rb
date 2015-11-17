# Takes an array of key value pairs and turns it into a hash of
# hashes. The keys on the first level may contain dots as this is how
# we'll output them with the to_toml function.
def from_assoc(array)
  parameters = {}
  array.each_slice(2) do |name, value|
    *namespaces, field = name.split(".")
    ns = namespaces.join(".")
    parameters[ns] ||= {}
    parameters[ns][field] = value
  end
  parameters
end

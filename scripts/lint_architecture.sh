#!/bin/bash
package_metadata=$(cargo metadata --format-version 1)

has_domain_invalid_dependency=$(echo "$package_metadata" |
	jq '.packages[] | select(.manifest_path | contains("crates/domain")) 
				| .dependencies[] 
				| select(.path | index("crates/"))')
if [[ -n "$has_domain_invalid_dependency" ]]; then
	echo "Domain lib dependencies violate hexagonal architecture"
	exit 1
else
	echo "Domain lib is fine"
fi

has_ports_invalid_dependency=$(echo "$package_metadata" |
	jq '.packages[] | select(.manifest_path | contains("crates/ports")) 
				| .dependencies[] 
				| select(
								(.path | index("crates/application")) or
								(.path | index("crates/adapters"))
						)')
if [[ -n "$has_ports_invalid_dependency" ]]; then
	echo "Ports lib dependencies violate hexagonal architecture"
	exit 1
else
	echo "Ports lib is fine"
fi

has_application_invalid_dependency=$(echo "$package_metadata" |
	jq '.packages[] | select(.manifest_path | contains("crates/application")) 
				| .dependencies[] 
				| select(.path | index("crates/adapters"))')
if [[ -n "$has_application_invalid_dependency" ]]; then
	echo "Application lib dependencies violate hexagonal architecture"
	exit 1
else
	echo "Application lib is fine"
fi

has_adapters_invalid_dependency=$(echo "$package_metadata" |
	jq '.packages[] | select(.manifest_path | contains("crates/adapters")) 
				| .dependencies[] 
				| select(.path | index("crates/application"))')
if [[ -n "$has_adapters_invalid_dependency" ]]; then
	echo "Adapters lib dependencies violate hexagonal architecture"
	exit 1
else
	echo "Adapters lib is fine"
fi

exit 0

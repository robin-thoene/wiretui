#!/bin/bash
package_metadata=$(cargo metadata --format-version 1)

has_domain_invalid_dependency=$(echo "$package_metadata" |
	jq '.packages[] | select(.name == "domain") 
				| .dependencies[] 
				| select(.path | index("src-libs/"))')
if [[ -n "$has_domain_invalid_dependency" ]]; then
	echo "Domain lib dependencies violate hexagonal architecture"
	exit 1
else
	echo "Domain lib is fine"
fi

has_ports_invalid_dependency=$(echo "$package_metadata" |
	jq '.packages[] | select(.name == "ports") 
				| .dependencies[] 
				| select(
								(.path | index("src-libs/application")) or
								(.path | index("src-libs/adapters"))
						)')
if [[ -n "$has_ports_invalid_dependency" ]]; then
	echo "Ports lib dependencies violate hexagonal architecture"
	exit 1
else
	echo "Ports lib is fine"
fi

has_application_invalid_dependency=$(echo "$package_metadata" |
	jq '.packages[] | select(.name == "application") 
				| .dependencies[] 
				| select(.path | index("src-libs/adapters"))')
if [[ -n "$has_application_invalid_dependency" ]]; then
	echo "Application lib dependencies violate hexagonal architecture"
	exit 1
else
	echo "Application lib is fine"
fi

exit 0

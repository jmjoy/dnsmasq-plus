#ifndef __LIBDNSMASQPLUS_H__
#define __LIBDNSMASQPLUS_H__

/**
 * Parse and add regexp to global.
 */
extern int dnsmasq_plus_global_add_regex(const char *regexp);

/**
 * Use regexp to match a domain.
 */
extern int dnsmasq_plus_hostname_is_match(const char *regexp, const char *query_domain);


#endif
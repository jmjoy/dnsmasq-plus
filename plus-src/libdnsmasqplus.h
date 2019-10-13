#ifndef __LIBDNSMASQPLUS_H__
#define __LIBDNSMASQPLUS_H__

/**
 * Parse and add regexp to global.
 */
extern void *dnsmasq_plus_parse_regex(const char *regexp);

/**
 * Use regexp to match a domain.
 */
extern int dnsmasq_plus_hostname_is_match(const char *regex, const char *query_domain);


#endif
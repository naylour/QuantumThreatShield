import { deLocalizeUrl } from '$i18n/runtime';

export const reroute = (request) => deLocalizeUrl(request.url).pathname;

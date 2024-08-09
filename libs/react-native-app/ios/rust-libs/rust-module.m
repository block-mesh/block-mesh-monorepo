#import <React/RCTBridgeModule.h>
#include "blockmesh-cli.h"

@interface RustModule : NSObject <RCTBridgeModule>
@end

@implementation RustModule

RCT_EXPORT_MODULE();

RCT_EXPORT_METHOD(runLib:(NSString *)url
                  email:(NSString *)email
                  password:(NSString *)password
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject) {
    const char *cUrl = [url UTF8String];
    const char *cEmail = [email UTF8String];
    const char *cPassword = [password UTF8String];
    int result = run_lib(cUrl, cEmail, cPassword);
    resolve(@(result));
}

RCT_EXPORT_METHOD(stopLib:(RCTPromiseResolveBlock)resolve
                      rejecter:(RCTPromiseRejectBlock)reject) {
    int result = stop_lib();
    resolve(@(result));
}

@end

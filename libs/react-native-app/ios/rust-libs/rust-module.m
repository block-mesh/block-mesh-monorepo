#import <React/RCTBridgeModule.h>
#include "blockmesh-cli.h"

@interface RustModule : NSObject <RCTBridgeModule>
@property (nonatomic, strong) NSOperationQueue *operationQueue ;
@end

@implementation RustModule

RCT_EXPORT_MODULE();

RCT_EXPORT_METHOD(runLib:(NSString *)url
                  email:(NSString *)email
                  password:(NSString *)password
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject) {
    if (!self.operationQueue) {
        self.operationQueue = [[NSOperationQueue alloc] init];
        self.operationQueue.suspended = NO;
    }
    const char *cUrl = [url UTF8String];
    const char *cEmail = [email UTF8String];
    const char *cPassword = [password UTF8String];
    NSLog(@"run_lib starts");
    // Create and start the NSThread
//     dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
//     [NSThread detachNewThreadWithBlock:^{
    NSOperation *operation = [NSBlockOperation blockOperationWithBlock:^{

        NSLog(@"run_lib starts 2");
//         NSLog(@"url = %@ , email = %@ , password = %@", url, email, password);
        int result = run_lib(cUrl, cEmail, cPassword);
        NSLog(@"run_lib starts result = %@", result);
        NSLog(@"run_lib ends");
//     }];
//     });
}];
    [self.operationQueue addOperation:operation];
    resolve(@(1));
}

RCT_EXPORT_METHOD(stopLib:(RCTPromiseResolveBlock)resolve
                      rejecter:(RCTPromiseRejectBlock)reject) {
    NSLog(@"stop_lib starts");
    [self.operationQueue cancelAllOperations];
    NSLog(@"stop_lib end");
//     dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
//     NSLog(@"stop_lib starts");
//     stop_lib();
//     NSLog(@"stop_lib end");
//     resolve(@(1));
//     });
}

@end

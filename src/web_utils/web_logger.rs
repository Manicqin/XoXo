use iron::prelude::*;
use iron::{Handler, AroundMiddleware};

pub enum LoggerMode {
    Silent,
    Tiny,
    Large
}

pub struct Logger {
    mode: LoggerMode
}

pub struct LoggerHandler<H: Handler> { logger: Logger, handler: H }

impl Logger {
    pub fn new(mode: LoggerMode) -> Logger {
        Logger { mode: mode }
    }

    fn log(&self, req: &Request, res: Result<&Response, &IronError>, time: u64) {
        match self.mode {
            LoggerMode::Silent => {},
            LoggerMode::Tiny => println!("Req: {:?}\nRes: {:?}\nTook: {}", req, res, time),
            LoggerMode::Large => println!("Request: {:?}\nResponse: {:?}\nResponse-Time: {}", req, res, time)
        }
    }

    pub fn parse_logger_mode(log_level: u8)-> LoggerMode{
        match log_level{
            0 => LoggerMode::Silent,
            1 => LoggerMode::Tiny,
            _ => LoggerMode::Large,
        }
    }
}

impl<H: Handler> Handler for LoggerHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let entry = ::time::precise_time_ns();
        let res = self.handler.handle(req);
        self.logger.log(req, res.as_ref(), ::time::precise_time_ns() - entry);
        res
    }
}

impl AroundMiddleware for Logger {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(LoggerHandler {
            logger: self,
            handler: handler
        }) as Box<Handler>
    }
}
